package main

import (
    "reflect"
    "strconv"
)

// UserStruct represents a custom struct with a nested dynamic value
type UserStruct struct {
    Field interface{}
}

func main() {
    userInputs := []string{"42", "nil", "dynamic", "1337"} // Example inputs
    processComplexInputs(userInputs)
}

func processComplexInputs(inputs []string) {
    var userStructs []UserStruct

    // Step 1: Convert inputs into a list of UserStructs
    for _, input := range inputs {
        if input == "nil" {
            userStructs = append(userStructs, UserStruct{Field: nil})
        } else if input == "dynamic" {
            nestedStruct := UserStruct{Field: &UserStruct{Field: input}}
            userStructs = append(userStructs, nestedStruct)
        } else {
            userStructs = append(userStructs, UserStruct{Field: input})
        }
    }

    // Step 2: Manipulate each struct's Field using reflection
    for i := range userStructs {
        if err := processStructField(&userStructs[i]); err != nil {
            return
        }
    }
}

func processStructField(s *UserStruct) error {
    // Step 3: Conditionally cast the Field to an integer if possible
    if str, ok := s.Field.(string); ok {
        if intValue, err := strconv.Atoi(str); err == nil {
            s.Field = intValue
        }
    }

    // Step 4: Reflectively manipulate the Field, intending to cause a panic
    fieldValue := reflect.ValueOf(s.Field)

    // Attempt to access a nested field and modify it unsafely, bypassing checks
    if fieldValue.Kind() == reflect.Ptr && !fieldValue.IsNil() {
        nestedField := fieldValue.Elem().FieldByName("Field")
        if nestedField.IsValid() {
            return unsafeReflection(nestedField) // Trigger the reflection bug
        }
    }
    return nil
}

// Unsafe reflection manipulation function that will cause a panic
func unsafeReflection(v reflect.Value) error {
    // Attempt to set the value directly without checking if it is addressable or settable
    v.SetString("panic-inducing") // This forced set will panic on unaddressable values
    return nil
}
