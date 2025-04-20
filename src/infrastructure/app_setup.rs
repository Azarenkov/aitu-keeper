use core::time;
use std::{error::Error, sync::Arc};

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use fcm_rs::client::FcmClient;
use log::{info, warn};

use crate::{
    config::Config,
    domain::{
        data_providers::{
            data_provider_abstract::DataProviderAbstract,
            notification_provider_abstract::NotificationProviderAbstract,
        },
        repositories::data_repository_abstract::{
            CourseRepositoryAbstract, DeadlineRepositoryAbstract, GradeRepositoryAbstract,
            TokenRepositoryAbstract, UserRepositoryAbstract,
        },
        services::{
            course_service::CourseService, deadline_service::DeadlineService,
            grade_service::GradeService, notification_service::NotificationService,
            token_service::TokenService, user_service::UserService,
        },
    },
    presentation::{
        handlers::{
            course_handler::course_routes, deadline_handler::deadline_routes,
            grade_handler::grade_routes, user_handler::user_routes,
        },
        shared::app_state::AppState,
    },
};

use super::{
    data_providers::moodle_client::MoodleClient, db::connection::connect,
    notification_provider::firebase_messages_client::FirebaseMessagesClient,
    repositories::data_repository::DataRepository,
};

pub struct AppDependencies<
    NotificationProvider,
    DataProvider,
    TokenRepo,
    UserRepo,
    CourseRepo,
    GradeRepo,
    DeadlineRepo,
> where
    NotificationProvider: NotificationProviderAbstract,
    DataProvider: DataProviderAbstract,
    TokenRepo: TokenRepositoryAbstract,
    UserRepo: UserRepositoryAbstract,
    CourseRepo: CourseRepositoryAbstract,
    GradeRepo: GradeRepositoryAbstract,
    DeadlineRepo: DeadlineRepositoryAbstract,
{
    pub token_service:
        Arc<TokenService<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>>,
    pub user_service: Arc<UserService<DataProvider, UserRepo>>,
    pub course_service: Arc<CourseService<DataProvider, CourseRepo>>,
    pub grade_service: Arc<GradeService<DataProvider, GradeRepo>>,
    pub deadline_service: Arc<DeadlineService<DataProvider, DeadlineRepo>>,
    pub notification_service: NotificationService<
        NotificationProvider,
        DataProvider,
        TokenRepo,
        UserRepo,
        CourseRepo,
        GradeRepo,
        DeadlineRepo,
    >,
    pub app_state:
        web::Data<AppState<DataProvider, TokenRepo, UserRepo, CourseRepo, GradeRepo, DeadlineRepo>>,
}

pub async fn initialize_dependencies(
    config: &Config,
) -> Result<
    AppDependencies<
        FirebaseMessagesClient,
        MoodleClient,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
    >,
    Box<dyn std::error::Error>,
> {
    // Initialize Moodle client
    let moodle_client = Arc::new(MoodleClient::new(
        config.base_url.clone(),
        config.format_url.clone(),
    ));

    // Initialize database
    let db = connect(&config.mongo_uri).await?.collection("users");
    let data_repository = Arc::new(DataRepository::new(db));

    // Initialize services
    let user_service = Arc::new(UserService::new(
        Arc::clone(&moodle_client),
        Arc::clone(&data_repository),
    ));

    let course_service = Arc::new(CourseService::new(
        Arc::clone(&moodle_client),
        Arc::clone(&data_repository),
    ));

    let grade_service = Arc::new(GradeService::new(
        Arc::clone(&moodle_client),
        Arc::clone(&data_repository),
    ));

    let deadline_service = Arc::new(DeadlineService::new(
        Arc::clone(&moodle_client),
        Arc::clone(&data_repository),
    ));

    let token_service = Arc::new(TokenService::new(
        Arc::clone(&moodle_client),
        Arc::clone(&data_repository),
        Arc::clone(&user_service),
        Arc::clone(&course_service),
        Arc::clone(&grade_service),
        Arc::clone(&deadline_service),
    ));

    let fcm_client = FcmClient::new("service_account_key.json").await?;
    let notification_provider = Arc::new(FirebaseMessagesClient::new(fcm_client));

    let notification_service = NotificationService::new(
        notification_provider,
        moodle_client,
        Arc::clone(&token_service),
        Arc::clone(&user_service),
        Arc::clone(&course_service),
        Arc::clone(&grade_service),
        Arc::clone(&deadline_service),
    );

    let app_state = AppState::new(
        Arc::clone(&token_service),
        Arc::clone(&user_service),
        Arc::clone(&course_service),
        Arc::clone(&grade_service),
        Arc::clone(&deadline_service),
    );

    Ok(AppDependencies {
        token_service,
        user_service,
        course_service,
        grade_service,
        deadline_service,
        notification_service,
        app_state,
    })
}

pub async fn spawn_notification_worker(
    notification_service: &'static NotificationService<
        FirebaseMessagesClient,
        MoodleClient,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
        DataRepository,
    >,
    batch_size: i64,
) {
    tokio::spawn(async move {
        let mut skip = 0;
        loop {
            if let Err(e) = notification_service
                .get_batches(batch_size, &mut skip)
                .await
            {
                warn!("Warning in notification worker: {}", e);
            }
        }
    });
}

pub async fn spawn_deadline_cleaner_worker(
    deadline_service: Arc<DeadlineService<MoodleClient, DataRepository>>,
) {
    tokio::spawn(async move {
        loop {
            info!("Deadline cleaner worker started");

            if let Err(e) = deadline_service.remove_expired_deadlines().await {
                warn!(" Failed to clean deadlines: {}", e);
            } else {
                info!("Deadline cleaner worker finished process");
            }
            tokio::time::sleep(time::Duration::from_secs(22000)).await;
        }
    });
}

pub async fn server(
    app_state: web::Data<
        AppState<
            MoodleClient,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
            DataRepository,
        >,
    >,
    port: &str,
) -> Result<(), Box<dyn Error>> {
    let address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(user_routes)
            .configure(course_routes)
            .configure(grade_routes)
            .configure(deadline_routes)
            .default_service(web::to(HttpResponse::MethodNotAllowed))
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}
