pub enum Poll<T> {
    Ready(T),
    Pending
}

pub trait MyFuture {
    type Output;
    fn my_poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

pub struct MyJoin<A, B> {
    a: Option<A>,
    b: Option<B>,
}

impl<A, B> MyFuture for MyJoin<A, B> 
where 
    A: MyFuture<Output = ()>,
    B: MyFuture<Output = ()>
{
    type Output = ();

    fn my_poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.my_poll(wake) {
                self.a.take();
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.my_poll(wake) {
                self.b.take();
            }
        }
        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct MyAndThen<A, B> {
    first: Option<A>,
    second: B,
}

impl<A, B> MyFuture for MyAndThen<A, B>
where
    A: MyFuture<Output = ()>,
    B: MyFuture<Output = ()>
{
    type Output = ();

    fn my_poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.my_poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }
        self.second.my_poll(wake)
    }
}

fn main() {

}
