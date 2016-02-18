using System.Collections.Generic;
using MangaDb.Attributes;
using MangaDb.Configurations;

namespace MangaDb.Entities
{
    public class ListEntry
    {
        public string Url { get; set; }
        [Include] public string Name { get; set; }
        [Exclude] public string Genries { get; set; }
        [Include] public string Translation { get; set; }
        public string TumbnailUrl { get; set; }

        public ListEntry Init(Groups groups, List<string> values)
        {
            var type = GetType();

            groups.Group.ForEach(g => 
                type.GetProperty(g.PropName)
                    .SetValue(this, values[g.Id]));

            return this;
        }

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(null, obj)) return false;
            if (ReferenceEquals(this, obj)) return true;
            if (obj.GetType() != this.GetType()) return false;
            return Equals((ListEntry) obj);
        }

        protected bool Equals(ListEntry other)
        {
            return string.Equals(Name, other.Name);
        }

        public override int GetHashCode()
        {
            return (Url != null ? Url.GetHashCode() : 0);
        }

    }
}
