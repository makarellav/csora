build:
	@gcc ./src/*.c -Wall -std=c99 -lSDL2 -o main

clean:
	@rm ./main

run:
	@$(MAKE) --no-print-directory build
	@./main