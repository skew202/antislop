// Clean Go code without slop
package processor

import "strings"

type Config struct {
	RequiredFields []string
	MaxBatchSize   int
}

type Record struct {
	ID   string
	Name string
	Data map[string]interface{}
}

type ProcessedRecord struct {
	ID        string
	Name      string
	Processed bool
}

func CalculateFibonacci(n int) int {
	if n <= 1 {
		return n
	}

	prev, curr := 0, 1
	for i := 2; i <= n; i++ {
		prev, curr = curr, prev+curr
	}

	return curr
}

func MergeSortedSlices(slice1, slice2 []int) []int {
	result := make([]int, 0, len(slice1)+len(slice2))
	i, j := 0, 0

	for i < len(slice1) && j < len(slice2) {
		if slice1[i] <= slice2[j] {
			result = append(result, slice1[i])
			i++
		} else {
			result = append(result, slice2[j])
			j++
		}
	}

	result = append(result, slice1[i:]...)
	result = append(result, slice2[j:]...)
	return result
}

type DataProcessor struct {
	config         Config
	processedCount int
}

func NewDataProcessor(config Config) *DataProcessor {
	return &DataProcessor{
		config:         config,
		processedCount: 0,
	}
}

func (p *DataProcessor) Process(records []Record) []ProcessedRecord {
	results := make([]ProcessedRecord, 0)
	for _, record := range records {
		if p.isValid(record) {
			results = append(results, p.transform(record))
			p.processedCount++
		}
	}
	return results
}

func (p *DataProcessor) isValid(record Record) bool {
	for _, field := range p.config.RequiredFields {
		if _, ok := record.Data[field]; !ok {
			return false
		}
	}
	return true
}

func (p *DataProcessor) transform(record Record) ProcessedRecord {
	return ProcessedRecord{
		ID:        record.ID,
		Name:      strings.ToUpper(record.Name),
		Processed: true,
	}
}
