// examples/document_loaders.rs

use rig::{
    completion::Prompt,
    document_loaders::PdfLoader,
    embeddings::EmbeddingsBuilder,
    providers::openai::{Client, TEXT_EMBEDDING_ADA_002},
    vector_store::{in_memory_store::InMemoryVectorStore, VectorStore},
};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Print current working directory
    println!("Current working directory: {:?}", env::current_dir()?);

    // Path to the PDF file
    let pdf_path = PathBuf::from("rig-core/examples/sample_data/moores_law_for_everything.pdf");

    // Print absolute path
    println!(
        "Attempting to access file at: {:?}",
        pdf_path.canonicalize()?
    );

    // Check if the file exists
    if !pdf_path.exists() {
        eprintln!("Error: The file {} does not exist.", pdf_path.display());
        return Ok(());
    }

    println!("File found successfully!");

    // Initialize OpenAI client
    let openai = Client::from_env();
    let embedding_model = openai.embedding_model(TEXT_EMBEDDING_ADA_002);

    // Create vector store
    let mut vector_store = InMemoryVectorStore::default();

    // Build embeddings
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .add_loader(PdfLoader::new(pdf_path.to_str().unwrap()))
        .build()
        .await?;

    println!(
        "Embeddings created successfully. Count: {}",
        embeddings.len()
    );
    for emb in &embeddings {
        println!("Document ID: {}", emb.id);
        println!("Document Content: {:?}", emb.document);
        println!("Number of embeddings: {}", emb.embeddings.len());
        println!(
            "First embedding vector length: {}",
            emb.embeddings.first().map_or(0, |e| e.vec.len())
        );
        println!("--------------------");
    }

    // Add documents to vector store
    vector_store.add_documents(embeddings).await?;

    // Create vector store index
    let index = vector_store.index(embedding_model);

    // Create RAG agent
    let rag_agent = openai
        .agent("gpt-4")
        .preamble(
            "
            You are a knowledgeable assistant.
            Use the information provided to you to answer questions.
        ",
        )
        .dynamic_context(5, index)
        .build();

    // Prompt the agent and print the response
    let response = rag_agent
        .prompt("give me a summary of the document.")
        .await?;

    println!("Agent Response:\n{}", response);

    Ok(())
}