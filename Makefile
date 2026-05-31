build:
	@gcc -Wall -std=c99 ./src/*.c -o main

clean:
	@rm ./main

run:
	@$(MAKE) --no-print-directory build
	@./main