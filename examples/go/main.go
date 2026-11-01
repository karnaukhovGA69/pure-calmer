// Pure Calmer / Go — розовый, голубой, синий и лавандовый.
// Красный зарезервирован для ошибок. Комментарии — спокойные серые.
package main

import (
	"context"
	"errors"
	"fmt"
	"sort"
	"sync"
	"time"
)

// 01 / TYPES, INTERFACES, GENERICS, FIELDS, STRUCT TAGS
// Типы — синие; функции — голубые; поля — серо-голубые.
type Status uint8

const (
	Idle Status = iota
	Running
	Finished
	DefaultLimit = 3
)

type Job struct {
	ID     int      `json:"id"`
	Name   string   `json:"name"`
	Load   float64  `json:"load"`
	Status Status   `json:"status"`
	Labels []string `json:"labels,omitempty"`
}

type Runner interface {
	Run(context.Context) error
}

type Queue[T any] struct {
	items []T
	mu    sync.Mutex
}

func (q *Queue[T]) Push(item T) {
	q.mu.Lock()
	defer q.mu.Unlock()
	q.items = append(q.items, item)
}

func Map[T, U any](items []T, transform func(T) U) []U {
	result := make([]U, 0, len(items))
	for _, item := range items {
		result = append(result, transform(item))
	}
	return result
}

// 02 / METHODS, PARAMETERS, POINTERS, ERROR VALUES
// error — обычный тип, поэтому он НЕ красный.
func (j *Job) Run(ctx context.Context) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
		if j.Load < 0 || j.Load > 100 {
			return errors.New("load must be within 0..100")
		}
		j.Status = Running
		return nil
	}
}

// 03 / STRINGS, ESCAPES, NUMBERS, BOOLEANS, OPERATORS
func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	message := "Hello, пастельный мир!\n\tquoted: \"hello\""
	raw := `{"theme":"Pure Calmer","accent":"pink"}`
	letter, emoji := 'Ж', '🌙'
	integer, hex, binary := 42, 0xCAFE, 0b1010
	fraction, exponent := 3.14159, 6.02e23
	enabled := true && !false
	optional := (*Job)(nil)

	jobs := []Job{
		{ID: 1, Name: "night batch", Load: 18.5, Status: Idle},
		{ID: 2, Name: "forecast", Load: 42.0, Labels: []string{"cpu", "ram"}},
		{ID: 3, Name: "packing", Load: 73.5},
	}
	sort.Slice(jobs, func(i, k int) bool { return jobs[i].Load < jobs[k].Load })
	names := Map(jobs, func(job Job) string { return job.Name })
	lookup := map[string]int{"jobs": len(jobs), "limit": DefaultLimit}
	queue := Queue[Job]{}

	// 04 / CONTROL FLOW, CLOSURES, INTERFACES, TYPE SWITCHES
	for index := range jobs {
		job := &jobs[index]
		if err := job.Run(ctx); err != nil {
			fmt.Printf("job %d: %v\n", job.ID, err)
			continue
		}
		queue.Push(*job)
	}

	var runner Runner = &jobs[0]
	var value any = runner
	switch typed := value.(type) {
	case Runner:
		fmt.Printf("runner: %T\n", typed)
	case string:
		fmt.Println(typed)
	default:
		fmt.Println("unknown value")
	}

	// 05 / CHANNELS, GOROUTINES, SELECT
	updates := make(chan string, 1)
	go func() { updates <- "finished"; close(updates) }()
	for update := range updates {
		fmt.Printf("%s: %v\n", update, names)
	}

	fmt.Println(message, raw, letter, emoji, integer, hex, binary)
	fmt.Println(fraction, exponent, enabled, optional, lookup, len(queue.items))

	// 06 / ДИАГНОСТИКА: убери // у следующей строки для красной ошибки.
	// var broken int = "this is not a number"
	// TODO: это обычный комментарий, не ошибка и не красный акцент.
}
