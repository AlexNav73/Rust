using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Entities
{
    public class ListEntry
    {
        public string Url { get; set; }
        public string Genries { get; set; }
        public string Name { get; set; }
        public string Translation { get; set; }
        public string TumbnailUrl { get; set; }

        public ListEntry Init(List<string> values)
        {
            var props = GetType().GetProperties();
            for (int i = 0; i < props.Length; i++)
                props[i].SetValue(this, values[i + 2]);
            return this;
        }
    }
}
