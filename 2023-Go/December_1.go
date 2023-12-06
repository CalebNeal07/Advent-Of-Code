package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func toNum(str string) string {
	//	fmt.Println(str)
	switch str {
	case "zero":
		return "0"
	case "one":
		return "1"
	case "two":
		return "2"
	case "three":
		return "3"
	case "four":
		return "4"
	case "five":
		return "5"
	case "six":
		return "6"
	case "seven":
		return "7"
	case "eight":
		return "8"
	case "nine":
		return "9"
	default:
		return "0"
	}
}

func main() {
	total := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			break
		}

		fmt.Print(text + "\t")

		pattern := regexp.MustCompile("([0-9]|one|two|three|four|five|six|seven|eight|nine)")

		matches := pattern.FindStringSubmatch(text)

		fmt.Print(matches)

		_, err := strconv.Atoi(matches[0])

		var amount string

		if err != nil {
			amount = toNum(matches[0])
		} else {
			amount = matches[0]
		}

		if len(matches) == 1 {
			goto END
		}

		_, err = strconv.Atoi(matches[len(matches)-1])

		fmt.Print("\t" + matches[0] + "\t")
		fmt.Print(matches[len(matches)-1] + "\t")

		if err != nil {
			amount += toNum(matches[len(matches)-1])
		} else {
			amount += matches[len(matches)-1]
		}

		//strconv.Atoi(matches[0])
		/*
			var val string

			for _, character := range text {
				_, err := strconv.Atoi(string(character))
				if err != nil {
					continue
				}

				val = string(character)
				break
			}

			for i := len(text) - 1; i >= 0; i-- {
				_, err := strconv.Atoi(string(text[i]))
				if err != nil {
					continue
				}

				val += string(text[i])
				break
			}
		*/
		// amount, _ := strconv.Atoi(val)

	END:
		fmt.Println(amount)
		val, err := strconv.Atoi(amount)
		total += val
	}

	fmt.Println(total)
}
