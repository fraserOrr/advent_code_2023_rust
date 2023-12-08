import sys
import re



def main():
  f = open("input_1.txt","r")
  total = 0
  match_words_regex = "(?=(one|two|three|four|five|six|seven|eight|nine|[1-9]))"

  for line in f:

    numbers = re.findall(match_words_regex,line)

    try:
   
      if len(numbers) > 1 :
        first_number = word_to_number(numbers[0])
        last_number = word_to_number(numbers[-1])
        i = convert(first_number,last_number)
      else:
        
        #print("convert single to number")
        first_number =  word_to_number(numbers[0])
        last_number =  word_to_number(numbers[0])
        i = convert(first_number,last_number)

      total = total + int(i)  
       
      print(line)
      print("number to add: " + str(i))
      print("Total: " + str(total))
      print("")



    except:

      print("out of bounbds")
      print(line)


  print("Grand Total: " + str(total)) 
   
def word_to_number(word):
  match word:
    case "1":
      i = 1
    case "one":
      i = 1
    case "2" :
      i = 2
    case "two" :
      i = 2      
    case "3":
      i = 3
    case "three":
      i = 3   
    case "4" :
      i = 4         
    case "four" :
      i = 4
    case "5":
      i = 5      
    case "five":
      i = 5
    case "6" :
      i = 6
    case "six" :
      i = 6   
    case "7":
      i = 7         
    case "seven":
      i = 7
    case "8" :
      i = 8      
    case "eight" :
      i = 8
    case "9" :
      i = 9      
    case "nine" :
      i = 9
  return(i)

def convert(first_number,last_number):

  i = int(str(str(first_number) + str(last_number)))
  return(i)




main()