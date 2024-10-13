import json
import re

inFile = open("test.txt", "r")
outFile = open("out.json", "w")

x = {}

i = 0
with inFile and outFile:
    for line in inFile:
        if i == 0:
            headers = re.split(r'\t', line.replace('\n', ''))
            
            print(headers)
        else:
            lineArr = re.split(r'\t', line)
            x["name"] = lineArr[0]
            x["country"] = lineArr[1]
            x["data"] = {}
            for j in range(2, len(headers), 1):
                x["data"][headers[j]] = lineArr[j].replace('\n', '')
            if i == 1:
                print(x)
                outFile.write("[")
            temp = json.dumps(x, indent=4)
            
            outFile.write(temp)
            outFile.write(',')
            outFile.write("\n")
        i += 1

    outFile.write("]")