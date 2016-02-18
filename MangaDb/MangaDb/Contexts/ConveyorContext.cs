using MangaDb.Configurations;
using MangaDb.Entities;
using MangaDb.Repositories;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Contexts
{
    public class ConveyorContext
    {
        public MainConfig Config { get; set; }
        public IRepository<ListEntry> Repository { get; set; }
    }
}
