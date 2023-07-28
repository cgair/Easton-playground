// Signal handlers are functions that are executed 
// when certain signals are sent to a process.
#include <unistd.h>
#include <stdio.h>
#include <signal.h>

static void sigShutdownHandler(int sig) {
    char* msg;
    switch (sig)
    {
    case SIGINT:
        msg = "Received SIGINT scheduling shutdown...";
        break;
    case SIGTERM:
        msg = "Received SIGTERM scheduling shutdown...";
        break;
    default:
        msg = "Received shutdown signal, scheduling shutdown...";
    }

    printf("%s\n", msg);
    // Perform cleanup tasks if needed

    // Exit the program
    exit(0);
}

void setupSignalHandlers(void) {
    struct sigaction act;
    act.sa_handler = sigShutdownHandler; // Assign the signal handling function
    sigemptyset(&act.sa_mask);           // Clear the signal mask during signal handling

    sigaction(SIGTERM, &act, NULL);
    // SIGINT (Signal Interrupt): 
    // This signal is sent when 
    // the user presses the interrupt character 
    // (usually Ctrl+C) in the terminal. 
    sigaction(SIGINT, &act, NULL);
}

void initServer() {
    // sets the signal handler for the xxx signal to 
    // SIG_IGN means the signal will be ignored. 

    // The SIGHUP signal is typically sent to a process 
    // when its controlling terminal is closed 
    // or the session leader terminates.
    signal(SIGHUP, SIG_IGN);

    // The SIGPIPE signal is sent to a process 
    // when it tries to write to a pipe or socket 
    // whose reading end has been closed.
    signal(SIGPIPE, SIG_IGN);

    setupSignalHandlers();

    for(;;) {
        sleep(5);
    }
}

int main()
{
    initServer();

    return 0;
}

// See also:
// <https://www.geeksforgeeks.org/signals-c-language/>