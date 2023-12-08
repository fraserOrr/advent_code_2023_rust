import sys
import re



def main():
  f = open("input_1.txt","r")
  total = 0
  for line in f:
		#print(line)
    numbers = re.findall("[0-9]{1}",line)
    print(numbers)
    if len(numbers) > 1 :
      first_number = numbers[0]
      last_number = numbers[-1]
      i = convert(first_number,last_number)
    else:
      #print("convert single to number")
      first_number = numbers[0]
      last_number = numbers[0]
      i = convert(first_number,last_number)
    total = total + int(i) 

  print(str(total))
 
def convert(first_number,last_number):

  i = int(str(str(first_number) + str(last_number)))
  return(i)



main()