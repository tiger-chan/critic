# Critic
A terminal-based application to compare and rate items (e.g., video games,
movies) using customizable criteria. Designed for ease of navigation and
quick decision-making.

## Features
Comparison and Rating: Compare two items side-by-side with options to skip,
 declare equal, or favor one over the other.
Navigation: Intuitive keyboard controls using arrow keys or WASD.
File Integration: Reads data from a user-provided database (e.g., ~/games.db).

## Rate
The screen allows users to compare two items under a specific category,
such as "General - Story."

### Options

- `Skip`: Skips the current comparison without affecting the rating
- `Equal`: Indicates that both are equivalent
- `Left-Option`: The first item being compared
- `Right-Option`: The second item being compared

## Group and Criteria Management
This screen allows users to organize and customize their groups and criteria:

- Groups: Represents categories such as "Action Adventure," "Soulslike," or
"Metroidvania."
- Criteria: Subcategories like "Combat Feel," "Exploration," or "Puzzle Design."

### Features of This Screen
Users can create, rename, or delete groups and criteria dynamically.
Intuitive navigation and editing for fine-tuning categories and criteria.

## Title Management
This screen allows users to manage titles (e.g., games, movies, or other media)
and assign them to specific groups.

### Features
- Title List: Displays all available titles. Navigate through the list to select a title for editing.
- Group Assignment: Assign the selected title to one or more groups by toggling the group checkboxes.

## Top Ratings
This screen allows users to view the current ratings of all titles in the
database.

## Installation
You can install critic in one of two ways:

1. Install directly using cargo install from the Git repository:
Run the following command to install the latest version directly from the repository:

```bash
cargo install --git https://github.com/tiger-chan/critic.git
```

2. Clone the repository and install manually:
If you'd prefer to clone the repository first:

    1. Clone the repository:
    ```bash
    git clone https://github.com/tiger-chan/critic.git
    cd critic
    ```

    2. Build and install the project:
    ```bash
    cargo install --path .
    ```
