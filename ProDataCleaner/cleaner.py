import json
import re
from bson import ObjectId

inFile = open("test.txt", "r")
outFile = open("../PlayerDataJSON/top_players.json", "w")

x = {"players" : []}

i = 0
with inFile and outFile:
    for line in inFile:
        if i == 0:
            headers = re.split(r'\t', line.replace('\n', ''))
            print(headers)
        else:
            lineArr = re.split(r'\t', line)
            tempMap = {}
            tempMap["_id"] = str(ObjectId())
            for j in range(len(headers)):
                tempMap[headers[j]] = lineArr[j].replace('\n', '')
            x["players"].append(tempMap)
            if i == 1:
                print(x)
        i += 1

    temp = json.dumps(x, indent=4)
    outFile.write(temp)
    outFile.write(',\n')
    outFile.write("\"curr_index\" : 0")