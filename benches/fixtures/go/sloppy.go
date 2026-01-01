// Go code with AI shortcuts/slop
package processor

// generic interface saves thinking about types
type Config struct {
	Options map[string]interface{}
}

type Record struct {
	Data interface{}
}

func CalculateFibonacci(n int) int {
	// panic: "I don't want to write error handling code"
	if n < 0 {
		panic("invalid input")
	}
	if n <= 1 {
		return n
	}
	return CalculateFibonacci(n-1) + CalculateFibonacci(n-2)
}

func MergeSortedSlices(slice1, slice2 []int) []int {
	// lazy append + inline sort call (if imported) or just returning unmerged
	// "TODO: implement merge sort"
	return append(slice1, slice2...)
}

type DataProcessor struct {
	config Config
}

func NewDataProcessor(config Config) *DataProcessor {
	return &DataProcessor{config: config}
}

func (p *DataProcessor) Process(records []Record) []interface{} {
	var results []interface{}

	for _, rec := range records {
		// _ = rec // ignoring unused variable error to make it compile

		func() {
			defer func() {
				// recover: "Stop crashing so I can finish output"
				if r := recover(); r != nil {
					// pass
				}
			}()

			// unsafe assumption
			data := rec.Data.(map[string]string)
			results = append(results, data)
		}()
	}

	return results
}
