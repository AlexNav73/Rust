using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Entities
{
    public class ListEntry
    {
        public string Name { get; set; }
        public List<string> Genries { get; set; }
        public string Translation { get; set; }
        public string TumbnailUrl { get; set; }


        public static List<string> ParseGenries(string list)
        {
            return list.Split(new char[] { ',', ' ' }, StringSplitOptions.RemoveEmptyEntries).ToList();
        }
    }
}
