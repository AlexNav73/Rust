using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Formatters
{
    public interface IFormatter<T>
    {
        string Format(IEnumerable<T> item);
    }
}
