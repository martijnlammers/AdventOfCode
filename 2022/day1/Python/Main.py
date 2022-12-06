data = open(".\\..\\input.txt", "r").read().splitlines()
totalCalories, highestCalories = 0, 0
for elfCalories in data:
    totalCalories = 0 if (elfCalories == "") else (totalCalories + int(elfCalories))
    if(totalCalories > highestCalories): highestCalories = totalCalories
print(highestCalories)