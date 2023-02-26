//Part 01:

int score = 0;

foreach (string l in System.IO.File.ReadAllLines("input.txt"))
{
    int[] arr = new int[4];
    string[] tt = l.Split(",");
    arr[0] = Int32.Parse(tt[0].Split("-")[0]);
    arr[1] = Int32.Parse(tt[0].Split("-")[1]);
    arr[2] = Int32.Parse(tt[1].Split("-")[0]);
    arr[3] = Int32.Parse(tt[1].Split("-")[1]);
    if ((arr[0] <= arr[2] && arr[3] <= arr[1]) || (arr[2] <= arr[0] && arr[1] <= arr[3]))
    {
        score++;
    }
}

Console.WriteLine(score);

//Part 02:

