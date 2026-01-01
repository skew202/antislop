// Example Go code with detected AI shortcuts
package main

func Process(data interface{}) {
	// type assertion without check
	m := data.(map[string]interface{})

	// panic for control flow: "lazy error handling"
	if m == nil {
		panic("nil map")
	}

	// TODO: implement logic
}

func HackyFix() {
	defer func() {
		// recover from panic and silence it: "make it run"
		if r := recover(); r != nil {
			// shhh
		}
	}()

	// stub
}
