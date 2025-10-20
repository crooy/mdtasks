use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

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
    /// Mark a task as started/active
    Start {
        /// Task ID to mark as started
        id: String,
    },
    /// Add an item to a task's checklist
    Checklist {
        /// Task ID to add checklist item to
        id: String,
        /// Checklist item to add
        item: String,
    },
    /// List subtasks (checklist items) for a task
    Subtasks {
        /// Task ID to list subtasks for
        id: String,
    },
    /// Set task title
    SetTitle {
        /// Task ID to update
        id: String,
        /// New title
        title: String,
    },
    /// Set task priority
    SetPriority {
        /// Task ID to update
        id: String,
        /// New priority
        priority: String,
    },
    /// Set task tags
    SetTags {
        /// Task ID to update
        id: String,
        /// New tags (comma-separated)
        tags: String,
    },
    /// Set task due date
    SetDue {
        /// Task ID to update
        id: String,
        /// New due date (YYYY-MM-DD)
        due: String,
    },
    /// Add note to task
    AddNote {
        /// Task ID to add note to
        id: String,
        /// Note to add
        note: String,
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
    completed: Option<String>,
    started: Option<String>,
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
        Commands::List {
            status,
            tag,
            priority,
        } => {
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
            notes,
        } => {
            add_task(title, priority, status, tags, project, due, notes)?;
        }
        Commands::Done { id } => {
            mark_task_done(id)?;
        }
        Commands::Start { id } => {
            mark_task_start(id)?;
        }
        Commands::Checklist { id, item } => {
            add_checklist_item(id, item)?;
        }
        Commands::Subtasks { id } => {
            list_subtasks(id)?;
        }
        Commands::SetTitle { id, title } => {
            set_task_field(id, "title", title)?;
        }
        Commands::SetPriority { id, priority } => {
            set_task_field(id, "priority", priority)?;
        }
        Commands::SetTags { id, tags } => {
            set_task_field(id, "tags", tags)?;
        }
        Commands::SetDue { id, due } => {
            set_task_field(id, "due", due)?;
        }
        Commands::AddNote { id, note } => {
            add_task_note(id, note)?;
        }
    }

    Ok(())
}

fn list_tasks(
    status_filter: Option<String>,
    tag_filter: Option<String>,
    priority_filter: Option<String>,
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
                    if !tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&tag.to_lowercase()))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Priority filter
            if let Some(ref priority) = priority_filter {
                if let Some(ref task_priority) = task.priority {
                    if !task_priority
                        .to_lowercase()
                        .contains(&priority.to_lowercase())
                    {
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

    println!(
        "{:<4} {:<12} {:<8} {:<50}",
        "ID", "STATUS", "PRIORITY", "TITLE"
    );
    println!("{}", "-".repeat(80));

    for task_file in filtered_tasks {
        let task = &task_file.task;
        let status = task.status.as_deref().unwrap_or("unknown");
        let priority = task.priority.as_deref().unwrap_or("medium");
        let title = &task.title;

        println!(
            "{:<4} {:<12} {:<8} {:<50}",
            task.id, status, priority, title
        );
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
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
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
        completed: None,
        started: None,
    };

    if let Pod::Hash(hash) = pod {
        for (key, value) in hash {
            match key.as_str() {
                "id" => match value {
                    Pod::String(s) => task.id = s.clone(),
                    Pod::Integer(i) => task.id = i.to_string(),
                    _ => {}
                },
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
        completed: None,
        started: None,
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
    content.push('\n');

    // Create filename
    let filename = format!(
        "tasks/{}-{}.md",
        next_id,
        title
            .to_lowercase()
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
        new_content.push_str(&format!(
            "completed: {}\n",
            chrono::Utc::now().format("%Y-%m-%d")
        ));

        new_content.push_str("---\n\n");

        // Process the markdown content to mark all checklist items as complete
        let processed_content = mark_all_subtasks_complete(&parsed.content);
        new_content.push_str(&processed_content);

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content).context(format!(
            "Failed to write updated task file: {}",
            task_file.file_path
        ))?;

        println!("✅ Marked task {} as done: {}", id, task.title);
    } else {
        return Err(anyhow::anyhow!(
            "Could not parse front-matter from task file"
        ));
    }

    Ok(())
}

fn mark_task_start(id: String) -> Result<()> {
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

        // Update the status to "active"
        task.status = Some("active".to_string());

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

        // Add started date
        new_content.push_str(&format!(
            "started: {}\n",
            chrono::Utc::now().format("%Y-%m-%d")
        ));

        new_content.push_str("---\n\n");

        // Add the original markdown content
        new_content.push_str(&parsed.content);

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content).context(format!(
            "Failed to write updated task file: {}",
            task_file.file_path
        ))?;

        println!("🚀 Started task {}: {}", id, task.title);
    } else {
        return Err(anyhow::anyhow!(
            "Could not parse front-matter from task file"
        ));
    }

    Ok(())
}

fn add_checklist_item(id: String, item: String) -> Result<()> {
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

    if let Some(_front_matter) = parsed.data {
        // Rebuild the content with the checklist item added
        let mut new_content = String::new();

        // Add the front-matter section
        let lines: Vec<&str> = content.lines().collect();
        let mut front_matter_end = 0;

        for (i, line) in lines.iter().enumerate() {
            if i > 0 && line == &"---" {
                front_matter_end = i;
                break;
            }
        }

        // Add front-matter
        for line in lines.iter().take(front_matter_end + 1) {
            new_content.push_str(&format!("{}\n", line));
        }

        // Find the checklist section and add the item
        let mut in_checklist = false;
        let mut checklist_added = false;

        for line in parsed.content.lines() {
            new_content.push_str(&format!("{}\n", line));

            // Check if we're in the checklist section
            if line.trim().starts_with("## Checklist") {
                in_checklist = true;
            } else if in_checklist
                && line.trim().starts_with("##")
                && !line.trim().starts_with("###")
            {
                // We've moved to the next section, add the item before this line
                new_content.push_str(&format!("- [ ] {}\n", item));
                checklist_added = true;
                in_checklist = false;
            } else if in_checklist && line.trim().is_empty() && !checklist_added {
                // Empty line in checklist section, add the item
                new_content.push_str(&format!("- [ ] {}\n", item));
                checklist_added = true;
            }
        }

        // If we never found a place to add it, add it at the end
        if !checklist_added {
            new_content.push_str(&format!("- [ ] {}\n", item));
        }

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content).context(format!(
            "Failed to write updated task file: {}",
            task_file.file_path
        ))?;

        println!("✅ Added checklist item to task {}: {}", id, item);
    } else {
        return Err(anyhow::anyhow!(
            "Could not parse front-matter from task file"
        ));
    }

    Ok(())
}

fn mark_all_subtasks_complete(content: &str) -> String {
    let mut result = String::new();
    let mut in_checklist = false;

    for line in content.lines() {
        // Check if we're entering the checklist section
        if line.trim().starts_with("## Checklist") {
            in_checklist = true;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check if we're leaving the checklist section
        if in_checklist && line.trim().starts_with("##") && !line.trim().starts_with("###") {
            in_checklist = false;
        }

        // If we're in the checklist section, mark all items as complete
        if in_checklist {
            let trimmed = line.trim();
            if trimmed.starts_with("- [ ]") {
                // Replace incomplete checkbox with complete checkbox
                let item_text = trimmed.strip_prefix("- [ ]").unwrap_or(trimmed).trim();
                result.push_str(&format!("- [x] {}\n", item_text));
            } else {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

fn list_subtasks(id: String) -> Result<()> {
    let tasks = load_tasks()?;

    let task_file = tasks
        .into_iter()
        .find(|tf| tf.task.id == id)
        .context(format!("Task with ID '{}' not found", id))?;

    let content = std::fs::read_to_string(&task_file.file_path)
        .context(format!("Failed to read task file: {}", task_file.file_path))?;

    let task = &task_file.task;

    println!("📋 Subtasks for task {}: {}", id, task.title);
    println!();

    // Find and display checklist items
    let mut in_checklist = false;
    let mut has_items = false;

    for line in content.lines() {
        // Check if we're entering the checklist section
        if line.trim().starts_with("## Checklist") {
            in_checklist = true;
            continue;
        }

        // Check if we're leaving the checklist section
        if in_checklist && line.trim().starts_with("##") && !line.trim().starts_with("###") {
            break;
        }

        // If we're in the checklist section, look for checklist items
        if in_checklist {
            let trimmed = line.trim();
            if trimmed.starts_with("- [") {
                has_items = true;
                // Extract the item text (remove the checkbox part)
                let item_text = if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                    // Completed item
                    let text = trimmed
                        .strip_prefix("- [x]")
                        .or_else(|| trimmed.strip_prefix("- [X]"))
                        .unwrap_or(trimmed)
                        .trim();
                    format!("✅ {}", text)
                } else if trimmed.starts_with("- [ ]") {
                    // Incomplete item
                    let text = trimmed.strip_prefix("- [ ]").unwrap_or(trimmed).trim();
                    format!("⏳ {}", text)
                } else {
                    // Fallback for other formats
                    trimmed.to_string()
                };
                println!("  {}", item_text);
            }
        }
    }

    if !has_items {
        println!("  No subtasks found.");
    }

    Ok(())
}
fn set_task_field(id: String, field: &str, value: String) -> Result<()> {
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

        // Update the specific field
        match field {
            "title" => task.title = value.clone(),
            "priority" => task.priority = Some(value.clone()),
            "tags" => {
                let tags: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                task.tags = Some(tags);
            }
            "due" => task.due = Some(value.clone()),
            _ => return Err(anyhow::anyhow!("Unknown field: {}", field)),
        }

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
            if tags.len() == 1 {
                new_content.push_str(&format!("tags: [\"{}\"]\n", tags[0]));
            } else {
                new_content.push_str("tags: [");
                for (i, tag) in tags.iter().enumerate() {
                    if i > 0 {
                        new_content.push_str(", ");
                    }
                    new_content.push_str(&format!("\"{}\"", tag));
                }
                new_content.push_str("]\n");
            }
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

        new_content.push_str("---\n\n");

        // Add the original markdown content
        new_content.push_str(&parsed.content);

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content).context(format!(
            "Failed to write updated task file: {}",
            task_file.file_path
        ))?;

        println!("✅ Updated {} for task {}: {}", field, id, value);
    } else {
        return Err(anyhow::anyhow!(
            "Could not parse front-matter from task file"
        ));
    }

    Ok(())
}

fn add_task_note(id: String, note: String) -> Result<()> {
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
        let task = extract_task_from_pod(&front_matter)?;

        // Rebuild the file content
        let mut new_content = String::new();

        // Add front-matter (unchanged)
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
            if tags.len() == 1 {
                new_content.push_str(&format!("tags: [\"{}\"]\n", tags[0]));
            } else {
                new_content.push_str("tags: [");
                for (i, tag) in tags.iter().enumerate() {
                    if i > 0 {
                        new_content.push_str(", ");
                    }
                    new_content.push_str(&format!("\"{}\"", tag));
                }
                new_content.push_str("]\n");
            }
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

        new_content.push_str("---\n\n");

        // Process the markdown content to add the note
        let processed_content = add_note_to_content(&parsed.content, &note);
        new_content.push_str(&processed_content);

        // Write the updated file
        std::fs::write(&task_file.file_path, new_content).context(format!(
            "Failed to write updated task file: {}",
            task_file.file_path
        ))?;

        println!("✅ Added note to task {}: {}", id, note);
    } else {
        return Err(anyhow::anyhow!(
            "Could not parse front-matter from task file"
        ));
    }

    Ok(())
}

fn add_note_to_content(content: &str, note: &str) -> String {
    let mut result = String::new();
    let mut in_notes = false;
    let mut notes_added = false;

    for line in content.lines() {
        // Check if we're entering the notes section
        if line.trim().starts_with("## Notes") {
            in_notes = true;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check if we're leaving the notes section
        if in_notes && line.trim().starts_with("##") && !line.trim().starts_with("###") {
            // Add the note before leaving the section
            if !notes_added {
                result.push_str(&format!("{}\n\n", note));
                notes_added = true;
            }
            in_notes = false;
        }

        // If we're in the notes section, add the note after the first empty line
        if in_notes && line.trim().is_empty() && !notes_added {
            result.push_str(line);
            result.push('\n');
            result.push_str(&format!("{}\n", note));
            notes_added = true;
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // If we never found a notes section, add it
    if !notes_added {
        result.push_str("\n## Notes\n");
        result.push_str(&format!("{}\n", note));
    }

    result
}
