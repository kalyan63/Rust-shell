To Run the shell you can just run the executable "shell"

To build it first:
1) First add the path(specific to user) of the bin folder in src in the execute.rs file. (line 7 in execute.rs)
2) Now run the make.sh file to add all the binaries to the bin folder.
3) Then just use "cargo run" to run the main shell file. 

Features: 
1) A single piping is implemented.
2) Use > and >> symbols for output redirection.
3) It can run any executable files in your system. 
4) We have also implemented bash scripting, it can run ".txt" files with the above mentioned commands in the file. ex: "bash file.txt"

Authors:
1) Kalyan Reddy S
2) Shahid 
3) Harsha Vardhan
4) Nikhil Pittala