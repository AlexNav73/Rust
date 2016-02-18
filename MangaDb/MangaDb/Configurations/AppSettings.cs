using System;
using System.Collections.Generic;
using System.Configuration;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Configurations
{
    public class AppSettings
    {
        public static string MainConfigPath
        {
            get
            {
                return ConfigurationManager.AppSettings[SettingKeys.MainConfigPath];
            }
        }

        public static string DbFileName
        {
            get
            {
                return ConfigurationManager.AppSettings[SettingKeys.DbFileName];
            }
        }
    }
}
