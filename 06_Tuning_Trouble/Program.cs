string input = File.ReadAllLines("input.txt")[0];

int cursor = 0;
int output = 0;
while (output == 0)
{
    Dictionary<char, int> tempDick = new Dictionary<char, int>();
    for (int i = cursor; i < cursor+14; i++)
    {
        if (tempDick.ContainsKey(input[i]))
        {
            break;
        }
        tempDick.Add(input[i], 1);
    }
    if (tempDick.Keys.Count == 14)
    {
        output = cursor+14;
        break;
    }
    cursor++;
}

Console.WriteLine(output);