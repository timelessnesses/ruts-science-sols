c:
	gcc matrix/c/sol.c --out matrix.exe
	gcc prime_and_odd/c/sol.c --out prime_and_odd.exe
	gcc plots_and_devs/c/sol.c --out plots_and_devs.exe
	echo "Built matrix.exe, prime_and_odd.exe and plots_and_devs.exe!"
rust:
	cargo build -r --bin matrix
	cargo build -r --bin prime_and_odd
	cargo build -r --bin plots_and_devs
	echo "Built executables inside target folder!"
python:
	echo "Just run it with Python man"