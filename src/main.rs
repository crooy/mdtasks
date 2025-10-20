use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    git: GitConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitConfig {
    branch_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git: GitConfig {
                branch_prefix: "feature/".to_string(),
            },
        }
    }
}

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
    /// Start Git branch for task
    GitStart {
        /// Task ID to create branch for
        id: String,
    },
    /// Finish Git branch and merge to main
    GitFinish {
        /// Optional commit message (defaults to task title)
        message: Option<String>,
    },
    /// Show Git status and current task
    GitStatus,
    /// Clean up done tasks (delete task files)
    Cleanup {
        /// Confirm cleanup without prompting
        #[arg(short, long)]
        yes: bool,
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

fn load_config() -> Result<Config> {
    // For now, just return default config
    // TODO: Load from config file when we implement that feature
    Ok(Config::default())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = load_config()?;

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
        Commands::GitStart { id } => {
            git_start_branch(id, &config)?;
        }
        Commands::GitFinish { message } => {
            git_finish_branch(message, &config)?;
        }
        Commands::GitStatus => {
            git_status(&config)?;
        }
        Commands::Cleanup { yes } => {
            cleanup_done_tasks(yes)?;
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
fn git_start_branch(task_id: String, config: &Config) -> Result<()> {
    // First, check if we're in a git repository
    if !is_git_repo()? {
        return Err(anyhow::anyhow!("Not in a git repository"));
    }

    // Get the task details
    let tasks = load_tasks()?;
    let task = tasks
        .into_iter()
        .find(|tf| tf.task.id == task_id)
        .context(format!("Task with ID '{}' not found", task_id))?;

    // Check if we're on main branch
    let current_branch = get_current_branch()?;
    if current_branch != "main" {
        return Err(anyhow::anyhow!(
            "Must be on main branch to start a task branch. Current branch: {}",
            current_branch
        ));
    }

    // Check if there are unstaged changes and warn
    let has_unstaged = has_uncommitted_changes()?;
    if has_unstaged {
        println!("⚠️  Warning: You have unstaged changes that will be auto-stashed and restored");
    }

    // Pull latest changes from main with auto-stash (keeps changes)
    println!("🔄 Pulling latest changes from main...");
    run_git_command(&["pull", "--rebase", "--autostash", "origin", "main"])?;

    // Create branch name from task
    let branch_name = format!(
        "{}{}-{}",
        config.git.branch_prefix,
        task_id,
        task.task
            .title
            .to_lowercase()
            .replace(" ", "-")
            .replace(":", "")
            .replace(",", "")
            .replace(".", "")
            .replace("!", "")
            .replace("?", "")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>()
    );

    // Check if branch already exists
    if branch_exists(&branch_name)? {
        return Err(anyhow::anyhow!("Branch '{}' already exists", branch_name));
    }

    // Create and checkout new branch
    println!("🌿 Creating branch: {}", branch_name);
    run_git_command(&["checkout", "-b", &branch_name])?;

    // Update task status to active if it's pending
    if task.task.status.as_deref() == Some("pending") {
        println!("🚀 Marking task {} as active", task_id);
        run_terminal_cmd_internal(&["mdtasks", "start", &task_id])?;
    }

    println!(
        "✅ Started work on task {} in branch '{}'",
        task_id, branch_name
    );
    println!("📝 Task: {}", task.task.title);

    Ok(())
}

fn git_finish_branch(message: Option<String>, config: &Config) -> Result<()> {
    // Check if we're in a git repository
    if !is_git_repo()? {
        return Err(anyhow::anyhow!("Not in a git repository"));
    }

    let current_branch = get_current_branch()?;

    // Check if we're on a task branch
    if !current_branch.starts_with(&config.git.branch_prefix) {
        return Err(anyhow::anyhow!(
            "Not on a task branch. Current branch: {}",
            current_branch
        ));
    }

    // Get task ID from branch name
    let task_id = current_branch
        .strip_prefix(&config.git.branch_prefix)
        .ok_or_else(|| anyhow::anyhow!("Invalid task branch format"))?
        .split('-')
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid task branch format"))?;

    // Get task details
    let tasks = load_tasks()?;
    let task = tasks
        .into_iter()
        .find(|tf| tf.task.id == task_id)
        .context(format!("Task with ID '{}' not found", task_id))?;

    // Mark task as done first (so the task file update gets committed)
    println!("✅ Marking task {} as done", task_id);
    run_terminal_cmd_internal(&["mdtasks", "done", task_id])?;

    // Commit message
    let commit_msg =
        message.unwrap_or_else(|| format!("feat: {} (task #{})", task.task.title, task_id));

    // Add all changes and commit (including the task file update)
    println!("📝 Committing changes...");
    run_git_command(&["add", "."])?;
    run_git_command(&["commit", "-m", &commit_msg])?;

    // Switch to main
    println!("🔄 Switching to main branch...");
    run_git_command(&["checkout", "main"])?;

    // Merge the task branch
    println!("🔀 Merging branch '{}' into main...", current_branch);
    run_git_command(&["merge", "--no-ff", &current_branch])?;

    // Delete the task branch
    println!("🗑️ Deleting task branch '{}'...", current_branch);
    run_git_command(&["branch", "-d", &current_branch])?;

    // Push changes to remote
    println!("🚀 Pushing changes to remote...");
    run_git_command(&["push", "origin", "main"])?;

    println!(
        "🎉 Successfully finished task {}: {}",
        task_id, task.task.title
    );
    println!("✅ Changes pushed to remote repository");

    Ok(())
}

fn git_status(config: &Config) -> Result<()> {
    // Check if we're in a git repository
    if !is_git_repo()? {
        return Err(anyhow::anyhow!("Not in a git repository"));
    }

    let current_branch = get_current_branch()?;
    println!("🌿 Current branch: {}", current_branch);

    if current_branch.starts_with(&config.git.branch_prefix) {
        // Extract task ID from branch name
        if let Some(task_id) = current_branch
            .strip_prefix(&config.git.branch_prefix)
            .and_then(|s| s.split('-').next())
        {
            // Try to get task details
            if let Ok(tasks) = load_tasks() {
                if let Some(task) = tasks.into_iter().find(|tf| tf.task.id == task_id) {
                    println!("📋 Current task: {} - {}", task_id, task.task.title);
                    println!(
                        "📊 Status: {}",
                        task.task.status.as_deref().unwrap_or("unknown")
                    );
                    println!(
                        "⭐ Priority: {}",
                        task.task.priority.as_deref().unwrap_or("none")
                    );
                } else {
                    println!("⚠️ Task {} not found in tasks directory", task_id);
                }
            }
        }
    } else {
        println!("📋 No active task branch");
    }

    // Show git status
    println!("\n📊 Git status:");
    run_git_command(&["status", "--short"])?;

    Ok(())
}

// Helper functions

fn is_git_repo() -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("Failed to run git command")?;

    Ok(output.status.success())
}

fn get_current_branch() -> Result<String> {
    let output = run_git_command(&["branch", "--show-current"])?;
    Ok(output.trim().to_string())
}

fn branch_exists(branch_name: &str) -> Result<bool> {
    let output = run_git_command(&["branch", "--list", branch_name])?;
    Ok(!output.trim().is_empty())
}

fn has_uncommitted_changes() -> Result<bool> {
    let output = run_git_command(&["status", "--porcelain"])?;
    Ok(!output.trim().is_empty())
}

fn run_git_command(args: &[&str]) -> Result<String> {
    let output = std::process::Command::new("git")
        .args(args)
        .output()
        .context(format!("Failed to run git command: git {}", args.join(" ")))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Git command failed: {}", error_msg));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_terminal_cmd_internal(args: &[&str]) -> Result<()> {
    let status = std::process::Command::new(args[0])
        .args(&args[1..])
        .status()
        .context(format!("Failed to run command: {}", args.join(" ")))?;

    if !status.success() {
        return Err(anyhow::anyhow!("Command failed: {}", args.join(" ")));
    }

    Ok(())
}

fn cleanup_done_tasks(yes: bool) -> Result<()> {
    let tasks = load_tasks()?;
    let done_tasks: Vec<_> = tasks
        .into_iter()
        .filter(|task_file| task_file.task.status.as_deref() == Some("done"))
        .collect();

    if done_tasks.is_empty() {
        println!("✅ No done tasks to clean up");
        return Ok(());
    }

    println!("🗑️  Found {} done task(s) to clean up:", done_tasks.len());
    for task_file in &done_tasks {
        println!("  - {}: {}", task_file.task.id, task_file.task.title);
    }

    if !yes {
        print!("❓ Are you sure you want to delete these task files? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().to_lowercase().starts_with('y') {
            println!("❌ Cleanup cancelled");
            return Ok(());
        }
    }

    let mut deleted_count = 0;
    for task_file in done_tasks {
        if let Err(e) = std::fs::remove_file(&task_file.file_path) {
            eprintln!("⚠️  Failed to delete {}: {}", task_file.file_path, e);
        } else {
            println!("🗑️  Deleted: {}", task_file.file_path);
            deleted_count += 1;
        }
    }

    println!("✅ Cleaned up {} done task(s)", deleted_count);
    Ok(())
}
