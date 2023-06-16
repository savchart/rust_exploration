use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::marker::PhantomPinned;
use std::task::{Context, Poll};

#[derive(Debug)]
struct PostStack {
    name: String,
    slug: *const String,
    _marker: PhantomPinned,
}

impl PostStack {
    pub fn new(name: String) -> Self {
        Self {
            name,
            slug: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }
    pub fn collect_slug(self: Pin<&mut Self>) {
        let ref_slug = &self.name as *const String;
        let this = unsafe {
            self.get_unchecked_mut()
        };
        this.slug = ref_slug;
    }
    pub fn get_slug(self: Pin<&Self>) -> String {
        unsafe { &*(self.slug) }.replace(" ", "-").to_lowercase()
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Future for PostStack {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("{}", self);
        Poll::Ready(())
    }
}

impl Display for PostStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = unsafe { Pin::new_unchecked(self) };
        write!(f,
               "name:`{}` slug:`{}`",
               this.as_ref().get_name(), PostStack::get_slug(this)
        ).unwrap();
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn pin_to_stack() {
    let mut post1 = PostStack::new("Article First".into());
    let mut post2 = PostStack::new("Article Second".into());
    let mut post1 = unsafe { Pin::new_unchecked(&mut post1) };
    let mut post2= unsafe { Pin::new_unchecked(&mut post2) };
    PostStack::collect_slug(post1.as_mut());
    PostStack::collect_slug(post2.as_mut());
    post1.await;
    post2.await;
}

struct PostHeap {
    name: String,
    slug: *const  String,
    _marker: PhantomPinned,
}

impl PostHeap {
    pub fn new(name: String) -> Pin<Box<Self>>{
        let post = Self {
            name,
            slug: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(post);
        let self_ptr = &boxed.name as *const String;
        unsafe { boxed.as_mut().get_unchecked_mut().slug  = self_ptr };
        boxed
    }
    fn get_slug(&self) -> String{
        unsafe{ &*(self.slug) }.replace(" ", "-").to_lowercase()
    }
    fn get_name(&self) -> &str{
        &self.name
    }
}

impl Display for PostHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "name:`{}` slug:`{}`",
                self.get_name(), self.get_slug()
        ).unwrap();
        Ok(())
    }
}

impl Future for PostHeap {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        println!("{}", self);
        Poll::Ready(())
    }
}


#[tokio::main(flavor = "current_thread")]
pub async fn pin_to_heap() {
    let post1 = PostHeap::new("Article First".into());
    let post2 = PostHeap::new("Article Second".into());
    post1.await;
    post2.await;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pin_to_stack() {
        pin_to_stack();
    }
    #[test]
    fn test_pin_to_heap() {
        pin_to_heap();
    }
}