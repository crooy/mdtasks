use clap::{Parser, Subcommand};
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;
use anyhow::{Result, Context};

#[derive(Parser)]
#[command(name = "mdtasks")]
#[command(about = "Markdown task manager")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks
    List {
        /// Filter by status (pending, active, done, partial)
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter by priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,
    },
    /// Show task details
    Show {
        /// Task ID to show
        id: String,
    },
    /// Add a new task
    Add {
        /// Task title/description
        title: String,

        /// Task priority (low, medium, high)
        #[arg(short = 'r', long)]
        priority: Option<String>,

        /// Task status (pending, active, done)
        #[arg(short, long)]
        status: Option<String>,

        /// Tags for the task
        #[arg(short = 'g', long)]
        tags: Option<Vec<String>>,

        /// Project name
        #[arg(short = 'j', long)]
        project: Option<String>,

        /// Due date
        #[arg(short, long)]
        due: Option<String>,

        /// Additional notes/content
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// Mark a task as done
    Done {
        /// Task ID to mark as done
        id: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: String,
    title: String,
    status: Option<String>,
    priority: Option<String>,
    tags: Option<Vec<String>>,
    project: Option<String>,
    created: Option<String>,
    due: Option<String>,
}

#[derive(Debug)]
struct TaskFile {
    task: Task,
    file_path: String,
    content: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { status, tag, priority } => {
            list_tasks(status, tag, priority)?;
        }
        Commands::Show { id } => {
            show_task(id)?;
        }
        Commands::Add {
            title,
            priority,
            status,
            tags,
            project,
            due,
            notes
        } => {
            add_task(title, priority, status, tags, project, due, notes)?;
        }
        Commands::Done { id } => {
            mark_task_done(id)?;
        }
    }

    Ok(())
}

fn list_tasks(
    status_filter: Option<String>,
    tag_filter: Option<String>,
    priority_filter: Option<String>
) -> Result<()> {
    let tasks = load_tasks()?;

    // Filter tasks
    let filtered_tasks: Vec<_> = tasks
        .into_iter()
        .filter(|task_file| {
            let task = &task_file.task;

            // Status filter
            if let Some(ref status) = status_filter {
                if let Some(ref task_status) = task.status {
                    if !task_status.to_lowercase().contains(&status.to_lowercase()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Tag filter
            if let Some(ref tag) = tag_filter {
                if let Some(ref tags) = task.tags {
                    if !tags.iter().any(|t| t.to_lowercase().contains(&tag.to_lowercase())) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Priority filter
            if let Some(ref priority) = priority_filter {
                if let Some(ref task_priority) = task.priority {
                    if !task_priority.to_lowercase().contains(&priority.to_lowercase()) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            true
        })
        .collect();

    // Display tasks
    if filtered_tasks.is_empty() {
        println!("No tasks found matching the criteria.");
        return Ok(());
    }

    println!("{:<4} {:<12} {:<8} {:<50}", "ID", "STATUS", "PRIORITY", "TITLE");
    println!("{}", "-".repeat(80));

    for task_file in filtered_tasks {
        let task = &task_file.task;
        let status = task.status.as_deref().unwrap_or("unknown");
        let priority = task.priority.as_deref().unwrap_or("medium");
        let title = &task.title;

        println!("{:<4} {:<12} {:<8} {:<50}", task.id, status, priority, title);
    }

    Ok(())
}

fn show_task(id: String) -> Result<()> {
    let tasks = load_tasks()?;

    let task_file = tasks
        .into_iter()
        .find(|tf| tf.task.id == id)
        .context(format!("Task with ID '{}' not found", id))?;

    let task = &task_file.task;

    println!("Task: {}", task.title);
    println!("ID: {}", task.id);
    println!("Status: {}", task.status.as_deref().unwrap_or("unknown"));
    println!("Priority: {}", task.priority.as_deref().unwrap_or("medium"));

    if let Some(ref tags) = task.tags {
        println!("Tags: {}", tags.join(", "));
    }

    if let Some(ref project) = task.project {
        println!("Project: {}", project);
    }

    if let Some(ref created) = task.created {
        println!("Created: {}", created);
    }

    if let Some(ref due) = task.due {
        println!("Due: {}", due);
    }

    println!("\nContent:");
    println!("{}", task_file.content);

    Ok(())
}

fn load_tasks() -> Result<Vec<TaskFile>> {
    let matter = Matter::<gray_matter::engine::YAML>::new();
    let mut tasks = Vec::new();

    // Look for markdown files in tasks/ directory
    let tasks_dir = Path::new("tasks");
    if !tasks_dir.exists() {
        return Ok(tasks);
    }

    for entry in WalkDir::new(tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path();
        let content = std::fs::read_to_string(file_path)
            .context(format!("Failed to read file: {}", file_path.display()))?;

        let parsed = matter.parse(&content);

        if let Some(front_matter) = parsed.data {
            // Try to extract fields manually from Pod
            match extract_task_from_pod(&front_matter) {
                Ok(task) => {
                    tasks.push(TaskFile {
                        task,
                        file_path: file_path.to_string_lossy().to_string(),
                        content: parsed.content,
                    });
                }
                Err(_) => {
                    // Skip files that don't have valid task data
                }
            }
        }
    }

    // Sort by ID
    tasks.sort_by(|a, b| a.task.id.cmp(&b.task.id));

    Ok(tasks)
}

fn extract_task_from_pod(pod: &gray_matter::Pod) -> Result<Task> {
    use gray_matter::Pod;

    let mut task = Task {
        id: String::new(),
        title: String::new(),
        status: None,
        priority: None,
        tags: None,
        project: None,
        created: None,
        due: None,
    };

    if let Pod::Hash(hash) = pod {
        for (key, value) in hash {
            match key.as_str() {
                "id" => {
                    match value {
                        Pod::String(s) => task.id = s.clone(),
                        Pod::Integer(i) => task.id = i.to_string(),
                        _ => {}
                    }
                }
                "title" => {
                    if let Pod::String(s) = value {
                        task.title = s.clone();
                    }
                }
                "status" => {
                    if let Pod::String(s) = value {
                        task.status = Some(s.clone());
                    }
                }
                "priority" => {
                    if let Pod::String(s) = value {
                        task.priority = Some(s.clone());
                    }
                }
                "tags" => {
                    if let Pod::Array(arr) = value {
                        let mut tags = Vec::new();
                        for item in arr {
                            if let Pod::String(s) = item {
                                tags.push(s.clone());
                            }
                        }
                        task.tags = Some(tags);
                    }
                }
                "project" => {
                    if let Pod::String(s) = value {
                        task.project = Some(s.clone());
                    }
                }
                "created" => {
                    if let Pod::String(s) = value {
                        task.created = Some(s.clone());
                    }
                }
                "due" => {
                    if let Pod::String(s) = value {
                        task.due = Some(s.clone());
                    }
                }
                _ => {}
            }
        }
    }

    if task.id.is_empty() || task.title.is_empty() {
        return Err(anyhow::anyhow!("Missing required fields: id or title"));
    }

    Ok(task)
}

fn add_task(
    title: String,
    priority: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
    project: Option<String>,
    due: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    // Generate next ID
    let next_id = get_next_task_id()?;

    // Create task struct
    let task = Task {
        id: next_id.clone(),
        title: title.clone(),
        status: status.or(Some("pending".to_string())),
        priority: priority.or(Some("medium".to_string())),
        tags,
        project,
        created: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
        due,
    };

    // Create markdown content
    let mut content = String::new();

    // Add front-matter
    content.push_str("---\n");
    content.push_str(&format!("id: {}\n", task.id));
    content.push_str(&format!("title: \"{}\"\n", task.title));

    if let Some(ref status) = task.status {
        content.push_str(&format!("status: {}\n", status));
    }

    if let Some(ref priority) = task.priority {
        content.push_str(&format!("priority: {}\n", priority));
    }

    if let Some(ref tags) = task.tags {
        content.push_str("tags: [");
        for (i, tag) in tags.iter().enumerate() {
            if i > 0 {
                content.push_str(", ");
            }
            content.push_str(&format!("\"{}\"", tag));
        }
        content.push_str("]\n");
    }

    if let Some(ref project) = task.project {
        content.push_str(&format!("project: {}\n", project));
    }

    if let Some(ref created) = task.created {
        content.push_str(&format!("created: {}\n", created));
    }

    if let Some(ref due) = task.due {
        content.push_str(&format!("due: {}\n", due));
    }

    content.push_str("---\n\n");

    // Add markdown content
    content.push_str("# Task Details\n\n");

    if let Some(ref notes) = notes {
        content.push_str("## Notes\n");
        content.push_str(&format!("{}\n\n", notes));
    }

    content.push_str("## Checklist\n");
    content.push_str("- [ ] \n\n");

    // Create filename
    let filename = format!("tasks/{}-{}.md",
        next_id,
        title.to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
    );

    // Ensure tasks directory exists
    std::fs::create_dir_all("tasks")?;

    // Write file
    std::fs::write(&filename, content)
        .context(format!("Failed to write task file: {}", filename))?;

    println!("✅ Created task {}: {}", next_id, title);
    println!("📁 File: {}", filename);

    Ok(())
}

fn get_next_task_id() -> Result<String> {
    let tasks = load_tasks()?;

    let mut max_id = 0;
    for task_file in tasks {
        if let Ok(id_num) = task_file.task.id.parse::<u32>() {
            max_id = max_id.max(id_num);
        }
    }

    Ok(format!("{:03}", max_id + 1))
}

fn mark_task_done(id: String) -> Result<()> {
    // Find the task file
    let tasks = load_tasks()?;
    let task_file = tasks
        .into_iter()
        .find(|tf| tf.task.id == id)
        .context(format!("Task with ID '{}' not found", id))?;

    // Read the current file content
    let content = std::fs::read_to_string(&task_file.file_path)
        .context(format!("Failed to read task file: {}", task_file.file_path))?;

    // Parse the front-matter and content
    let matter = Matter::<gray_matter::engine::YAML>::new();
    let parsed = matter.parse(&content);

    if let Some(front_matter) = parsed.data {
        // Extract the task data
        let mut task = extract_task_from_pod(&front_matter)?;

        // Update the status to "done"
        task.status = Some("done".to_string());

        // Rebuild the file content
        let mut new_content = String::new();

        // Add updated front-matter
        new_content.push_str("---\n");
        new_content.push_str(&format!("id: {}\n", task.id));
        new_content.push_str(&format!("title: \"{}\"\n", task.title));

        if let Some(ref status) = task.status {
            new_content.push_str(&format!("status: {}\n", status));
        }

        if let Some(ref priority) = task.priority {
            new_content.push_str(&format!("priority: {}\n", priority));
        }

        if let Some(ref tags) = task.tags {
            new_content.push_str("tags: [");
            for (i, tag) in tags.iter().enumerate() {
                if i > 0 {
                    new_content.push_str(", ");
                }
                new_content.push_str(&format!("\"{}\"", tag));
            }
            new_content.push_str("]\n");
        }

        if let Some(ref project) = task.project {
            new_content.push_str(&format!("project: {}\n", project));
        }

        if let Some(ref created) = task.created {
            new_content.push_str(&format!("created: {}\n", created));
        }

        if let Some(ref due) = task.due {
            new_content.push_str(&format!("due: {}\n", due));
        }

        // Add completed date
        new_content.push_str(&format!("completed: {}\n", chrono::Utc::now().format("%Y-%m-%d")));

        new_content.push_str("---\n\n");

        // Add the original markdown content
        new_content.push_str(&parsed.content);

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content)
            .context(format!("Failed to write updated task file: {}", task_file.file_path))?;

        println!("✅ Marked task {} as done: {}", id, task.title);
    } else {
        return Err(anyhow::anyhow!("Could not parse front-matter from task file"));
    }

    Ok(())
}
