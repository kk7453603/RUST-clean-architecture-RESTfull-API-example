use server::create_app_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    tracing_subscriber::fmt::init();

    println!("🚀 Запуск сервера на адресе: 0.0.0.0:3000");

    // Создание приложения
    let app = create_app_router();

    // Создание TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("✅ Сервер запущен и ожидает подключений...");

    // Запуск сервера
    axum::serve(listener, app).await?;

    Ok(())
}
