using MangaDb.Configurations;
using MangaDb.Helpers;
using MangaDb.Modules;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb
{
    public class Conveyor
    {
        private List<IModule> _modules = new List<IModule>();


        public Conveyor RegisterModule(IModule module)
        {
            _modules.Add(module);
            return this;
        }

        public async Task<object> Process()
        {
            object context = GetMainConfig();
//            object context = null; // GetMainConfig();

            foreach (IModule module in _modules)
            {
                context = await module.Execute(context);
                if (context == null)
                    return null;
            }
            return context;
        }

        private MainConfig GetMainConfig()
        {
            return ConfigurationHelper.Deserialize<MainConfig>(SettingKeys.MainConfigFileName);
        }
    }
}
