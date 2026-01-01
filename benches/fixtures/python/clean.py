# clean Python code without slop
def calculate_fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number using dynamic programming."""
    if n <= 1:
        return n
    
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    
    return curr


def merge_sorted_lists(list1: list, list2: list) -> list:
    """Merge two sorted lists into a single sorted list."""
    result = []
    i, j = 0, 0
    
    while i < len(list1) and j < len(list2):
        if list1[i] <= list2[j]:
            result.append(list1[i])
            i += 1
        else:
            result.append(list2[j])
            j += 1
    
    result.extend(list1[i:])
    result.extend(list2[j:])
    return result


class DataProcessor:
    """Process and transform data records."""
    
    def __init__(self, config: dict):
        self.config = config
        self.processed_count = 0
    
    def process(self, records: list) -> list:
        """Process a list of records according to configuration."""
        results = []
        for record in records:
            if self._is_valid(record):
                transformed = self._transform(record)
                results.append(transformed)
                self.processed_count += 1
        return results
    
    def _is_valid(self, record: dict) -> bool:
        """Validate a single record."""
        required_fields = self.config.get("required_fields", [])
        return all(field in record for field in required_fields)
    
    def _transform(self, record: dict) -> dict:
        """Transform a single record."""
        return {
            "id": record.get("id"),
            "name": record.get("name", "").upper(),
            "processed": True,
        }
