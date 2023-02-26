string[] lines = File.ReadAllLines("input.txt");
int divider = Array.IndexOf(lines, string.Empty);
int numStacks = int.Parse(lines[divider - 1].Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries).Last());
Console.WriteLine(numStacks);

List<char>[] stacks = new List<char>[numStacks];

for (int i = Array.IndexOf(lines, lines[divider - 2]); i >= 0; i--)
{
    
}

// string[] temp = lines[3].Replace("[", "").Replace("]", "").Trim().Split(" ");
// foreach(string s in temp)
// {
//     Console.WriteLine(s);
// }