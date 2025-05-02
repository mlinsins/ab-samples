#!/bin/bash

PROJECTS=("project_verifier_client" "project_xz_tar")

# For each of these projects sitting in the root of the repo:
#   For each job count between 1,...,8 inclusive
#     - Create a new folder named $project_j$x where x is the job count
#     - Copy the contents of the project folder into the new folder
#     - Create a new build.sh file in the new folder where we replave `-j4` with `-j$x`
#     - Also create a new GitHub action .yml by copying the existing one and replacing the job count in it

# Loop through each project
for project in "${PROJECTS[@]}"; do
    # Loop through each job count
    for job_count in {1..8}; do
        # Create the new folder name
        new_folder="${project}_j${job_count}"

        # Remove any existing folder with the same name
        if [ -d "$new_folder" ]; then
            rm -rf "$new_folder"
            echo "[+] Removed existing folder: $new_folder"
        fi
        
        # Create the new folder
        mkdir -p "$new_folder"
        
        # Copy the contents of the project folder into the new folder
        cp -r "$project/"* "$new_folder/"
        cp "$project/.gitignore"* "$new_folder/.gitignore"
        
        # Create a new build.sh file in the new folder with modified job count
        sed "s/-j4/-j${job_count}/" "$new_folder/build.sh" > "$new_folder/build.sh.tmp"
        mv "$new_folder/build.sh.tmp" "$new_folder/build.sh"
        echo "[+] Created new folder $new_folder with modified build.sh"

        # Create a new GitHub action .yml file with modified job count
        existing_action_file=".github/workflows/${project}.yml"
        new_action_file=".github/workflows/${new_folder}.yml"
        if [ -f "$existing_action_file" ]; then
            cp "$existing_action_file" "$new_action_file"
            sed -i "s/-j4/-j${job_count}/" "$new_action_file"

            # Add a sufix "(j$x)"" to the first list of the file
            # For example "name: Project Verifier Client" becomes "name: Project Verifier Client (j1)"
            sed -i "0,/name:/s/name: \(.*\)/name: \1 (j${job_count})/" "$new_action_file"

            # Update the "working-directory:" fields
            sed -i "s/working-directory: \(.*\)/working-directory: .\/${new_folder}/" "$new_action_file"
            echo "[+] Created GitHub action file: $new_action_file"
            
        else
            echo "Warning: $existing_action_file does not exist. Skipping action file creation for $new_folder."
        fi
    done
done
