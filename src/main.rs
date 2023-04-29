use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(scenario!("Load_Rust_Health").register_transaction(transaction!(rust_health)))
        .register_scenario(scenario!("Load_Rust_Mandel_256").register_transaction(transaction!(rust_mandel_256)))
        .register_scenario(scenario!("Load_Rust_Mandel_5000").register_transaction(transaction!(rust_mandel_5000)))
        .register_scenario(scenario!("Load_Rust_Mandel_256_400_800_NoMemory").register_transaction(transaction!(rust_mandel_256_hw)))
        .register_scenario(scenario!("Load_Rust_Mandel_256_400_800_Memory").register_transaction(transaction!(rust_mandel_256_hw_memory)))
        .register_scenario(scenario!("Load_Java_Mandel_256").register_transaction(transaction!(java_mandel_256)))
        .register_scenario(scenario!("Load_Java_Mandel_5000").register_transaction(transaction!(java_mandel_5000)))
        .register_scenario(scenario!("Load_Java_Mandel_256_400_800_NoMemory").register_transaction(transaction!(java_mandel_256_hw_nomemory)))
        .register_scenario(scenario!("Load_Java_Mandel_256_400_800_Memory").register_transaction(transaction!(java_mandel_256_hw_memory)))
        .register_scenario(scenario!("Load_Python_Mandel_256_400_800_Memory").register_transaction(transaction!(python_mandel_256_hw_memory)))
        .execute()
        .await?;

    Ok(())
}

async fn rust_health(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel-rust/actuator/health").await?;

    let validate = &Validate::builder()
        .status(200)
        //.text("Gander")
        .build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn rust_mandel_256(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_rust/mandel_text/256").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn rust_mandel_5000(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_rust/mandel_text/5000").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn rust_mandel_256_hw(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_rust/mandel_text/256/400/800").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn rust_mandel_256_hw_memory(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_rust/mandel_text_memory/256/400/800").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn java_mandel_256(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_java/mandel_text/256").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn java_mandel_5000(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_java/mandel_text/5000").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn java_mandel_256_hw_nomemory(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_java/mandel_text/256/400/800").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn java_mandel_256_hw_memory(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_java/mandel_text_memory/256/400/800").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}

async fn python_mandel_256_hw_memory(user: &mut GooseUser) -> TransactionResult {
    let goose_metrics = user.get("mandel_python/mandel_text_memory/256/400/800").await?;

    let validate = &Validate::builder().status(200).build();

    validate_and_load_static_assets(user, goose_metrics, &validate).await?;

    Ok(())
}
