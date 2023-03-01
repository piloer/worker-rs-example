use serde_json::json;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("System is running"))
        .post_async("/", |_,_| async move  {

            Response::ok("Hello,World")
        })

        .run(req, env)
        .await
}
