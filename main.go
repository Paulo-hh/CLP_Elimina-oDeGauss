package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"
)

const MAXN = 2000

var (
	N   int
	A   [][]float64
	B   []float64
	X   []float64
)

func initializeInputs() {
	A = make([][]float64, N)
	B = make([]float64, N)
	X = make([]float64, N)

	rand.Seed(time.Now().UnixNano())
	for i := 0; i < N; i++ {
		A[i] = make([]float64, N)
		for j := 0; j < N; j++ {
			A[i][j] = rand.Float64()
		}
		B[i] = rand.Float64()
		X[i] = 0.0
	}
}

func printMatrix() {
	if N < 10 {
		fmt.Println("Matrix A:")
		for i := 0; i < N; i++ {
			fmt.Println(A[i])
		}
		fmt.Println("Vector B:", B)
	}
}

func printSolution() {
	if N < 100 {
		fmt.Println("Solution X:", X)
	}
}

func gauss() {
	for norm := 0; norm < N-1; norm++ {
		for row := norm + 1; row < N; row++ {
			multiplier := A[row][norm] / A[norm][norm]
			for col := norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier
			}
			B[row] -= B[norm] * multiplier
		}
	}

	for row := N - 1; row >= 0; row-- {
		X[row] = B[row]
		for col := N - 1; col > row; col-- {
			X[row] -= A[row][col] * X[col]
		}
		X[row] /= A[row][row]
	}
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run gauss.go <matrix_dimension>")
		os.Exit(1)
	}

	n, err := strconv.Atoi(os.Args[1])
	if err != nil || n < 1 || n > MAXN {
		fmt.Println("Invalid matrix size.")
		os.Exit(1)
	}
	N = n

	initializeInputs()
	printMatrix()

	start := time.Now()
	gauss()
	
	duration := time.Since(start)
	printSolution()
	
	fmt.Printf("Elapsed time: %v ms\n", duration.Milliseconds())

}
