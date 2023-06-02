use actix_web::{
    dev,
    web, 
    dev::ServiceRequest,
    Error,
    body::{
        BoxBody,
    },
};
use actix_web_lab::middleware::Next;

use crate::nako::{
    global::AppState,
    view as nako_view,
};

use crate::app::service::setting;

//  权限检测
pub async fn settings(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<dev::ServiceResponse<BoxBody>, Error> {
    let state = req.app_data::<web::Data<AppState>>().unwrap();

    let setting_data = setting::settings(&mut state.get_ref().clone()).await;

    nako_view::SETTINGS.with(|states| {
        states.borrow_mut().replace(setting_data.clone());
    });

    return next.call(req).await;
}
