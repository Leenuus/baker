## Design

1. you should only manage local repo with baker
2. never manually manage local repo, or it will break
3. Setup dirs for setup scripts
4. Give users way to deploy some softwares with corresponding dotfiles, link setup files and config files

## Baker is a dotfiles backuper

file_to_backup --> backup_file_path(bk_path)

bk_path --> local_repo_path
local_repo_path --> bk_path

bk_path --> alias
alias --> local_repo_path
alias --> bk_path

## TODO

1. Add software setup features
2. Add backup file alias features for quick edit
3. Add verbose mode and dry run mode
4. Add cd, new a shell
5. modify subcommands, give alias, give setup scripts, 

### setup

1. user shell
2. user platform
