using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Contexts;
using MangaDb.Repositories.Implementations;

namespace MangaDb.Modules.Implementations
{
    public class Reader : IModule
    {
        public async Task<object> Execute(object context)
        {
            var cont = (ConveyorContext) context;

            var repo = new RecordRepository();
            repo.Init(cont.Config);

            return repo.GetAll();
        }
    }
}
