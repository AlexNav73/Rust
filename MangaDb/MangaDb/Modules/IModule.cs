using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Modules
{
    public interface IModule
    {
        Task<object> Execute(object context);
    }
}
