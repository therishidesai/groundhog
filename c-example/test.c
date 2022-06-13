#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include "../groundhog-record/groundhog.h"

void test_send(int write_fd, const char* data, const char* node) {
	EventType s = SEND;
	printf("Test: %s\n", data);
	write(write_fd, data, strlen(data)+1);
	log_event(node, &s, data);
}

int test_recv(int read_fd, const char* node) {
	EventType r = RECV;
	char read_data[128];
	int nbytes = read(read_fd, read_data, 128);
	if (nbytes > 0) {
		printf("read %d: %s\n", nbytes, read_data);
		log_event(node, &r, read_data);
	}
	return nbytes;
}

int main() {
	//hello();

	char read_data[128];

	int pipe_fds[2], nbytes;

	if (pipe(pipe_fds) == -1) {
		printf("creating pipe failed !!!!\n");
		return 1;
	}

	pid_t pid = fork();
	if (pid == -1) {
		printf("failed to fork!!!!\n");
		return 1;
	} else if (pid == 0) {
		// child proc
		init("node_b");
		close(pipe_fds[0]);
		EventType t = TRACE;
		char test_data[128];
		for (int i=0; i < 10; i++) {
			sprintf(test_data, "test_data_%d", i);
			log_event("node_b", &t, "test_log");
			test_send(pipe_fds[1], test_data, "node_b");
			sleep(1);
		}
		exit(0);
	} else {
		init("node_a");
		close(pipe_fds[1]);
		EventType t = TRACE;
		int nbytes = test_recv(pipe_fds[0], "node_a");
		while (nbytes > 0) {
			log_event("node_a", &t, "test_log");
			nbytes = test_recv(pipe_fds[0], "node_a");
		}

		printf("Done Running Example\n");
	}


	return 0;
}
