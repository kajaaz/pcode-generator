#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <string.h>
#include <unistd.h>

// Data structure to pass arguments to threads
typedef struct {
    char *message;
    int thread_id;
} ThreadArgs;

// Thread function
void *print_message(void *args) {
    ThreadArgs *thread_args = (ThreadArgs *)args;
    printf("Thread %d says: %s\n", thread_args->thread_id, thread_args->message);

    // Simulate work by sleeping
    sleep(1);

    // Free dynamically allocated memory
    free(thread_args->message);
    free(thread_args);

    return NULL;
}

int main() {
    const int thread_count = 3;  // Number of threads to create
    pthread_t threads[thread_count];

    printf("Main thread: Starting complex Hello, World!\n");

    // Create multiple threads
    for (int i = 0; i < thread_count; i++) {
        // Dynamically allocate memory for thread arguments
        ThreadArgs *args = malloc(sizeof(ThreadArgs));
        if (!args) {
            perror("Failed to allocate memory for thread arguments");
            exit(EXIT_FAILURE);
        }

        // Prepare the message for the thread
        char *message = malloc(50);
        if (!message) {
            perror("Failed to allocate memory for message");
            exit(EXIT_FAILURE);
        }

        snprintf(message, 50, "Hello from thread %d!", i);
        args->message = message;
        args->thread_id = i;

        // Create the thread
        if (pthread_create(&threads[i], NULL, print_message, args) != 0) {
            perror("Failed to create thread");
            exit(EXIT_FAILURE);
        }
    }

    // Wait for all threads to finish
    for (int i = 0; i < thread_count; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("Failed to join thread");
        }
    }

    printf("Main thread: All threads finished. Goodbye, World!\n");
    return 0;
}
