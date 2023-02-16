// C file that calls the bash scripts through input parsing

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

int main(int argc, char* argv[]) {
    // Get CLI Arguments from CLI, check if 2 args passed in
    if (argc >= 2) { 
        // Call bash scripts depending on argv[1]
        if (strcmp(argv[1],"install") == 0) {
            // Navigate to CLI-Tool/tool to be in same directory as Cargo.toml 
            //chdir("CLI-Tool/tool");
            int installOutput = system("./scripts/install.sh");
            //printf(installOutput);
            exit(0); // Exit 0 on succcess
        }
        else if (strcmp(argv[1],"test") == 0) {
            int testOutput = system("./scripts/test.sh");   // Call test.sh
            //printf(testOutput);
            exit(0); // Exit 0 on succcess
        }
        else { // file path
            char cmd[1000] = "./scripts/rank.sh ../";
	    strcat(cmd, argv[1]); 
	    int rankOutput = system(cmd);   // Call rank.sh
            //printf(rankOutput); 
            exit(0); // Exit 0 on succcess
        }   
    }
    else {
        fprintf(stderr, "Error, wrong number input of inputs");
        exit(1); // Exit 1 with an error 
    }

    return 0;
}
