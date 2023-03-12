string[] lines = File.ReadAllLines("input.txt");
int divider = Array.IndexOf(lines, string.Empty);
int numStacks = int.Parse(lines[divider - 1].Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries).Last());

List<Stack<char>> stacks = new List<Stack<char>> {};
int stackCursor = 1;

//TODO: find the end size of stackcursor programmatically.
while (stackCursor < 37)
{
    Stack<char> temp = new Stack<char>();
    for (int i = Array.IndexOf(lines, lines[divider-2]); i >= 0; i--)
    {
        if (lines[i][stackCursor] != ' ')
        {
            temp.Push(lines[i][stackCursor]);
        }
    }
    stacks.Add(temp);
    stackCursor += 4;
}

List<List<int>> workOrder = new List<List<int>>();

//TODO: The output of this can be put directly into a function instead of instantiated first???
for (int i = Array.IndexOf(lines, lines[divider + 1]); i < lines.Count(); i++)
{
    List<int> temp = new List<int>();
    foreach (string s in lines[i].Split(" "))
    {
        int tempInt;
        if (Int32.TryParse(s, out tempInt))
        {
            temp.Add(tempInt);
        }
    }
    workOrder.Add(temp);
}

void TheGreatCrane(int moves, int from, int to)
{
    for (int i = 0; i < moves; i++)
    {
        // stacks[to-1].Push(stacks[from-1].Pop());
    }
}

foreach(List<int> w in workOrder)
{
    TheGreatCrane(w[0], w[1], w[2]);
}

string output = "";
foreach(Stack<char> s in stacks)
{
    output += s.First();
}

Console.WriteLine(output);