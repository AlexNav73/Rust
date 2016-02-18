using System;
using System.Collections.Generic;
using System.Configuration;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using System.Web.UI;

namespace MangaDb.Configurations
{
    public class AppSettings
    {
        public static string MainConfigPath
        {
            get { return ConfigurationManager.AppSettings[SettingKeys.MainConfigPath]; }
        }

        public static string DbFileName
        {
            get { return ConfigurationManager.AppSettings[SettingKeys.DbFileName]; }
        }

        public static string ImgWidth
        {
            get { return ConfigurationManager.AppSettings[SettingKeys.ImgWidth]; }
        }

        public static string ImgHeight
        {
            get { return ConfigurationManager.AppSettings[SettingKeys.ImgHeight]; }
        }

        public static Type EntryType
        {
            get { return Type.GetType(ConfigurationManager.AppSettings[SettingKeys.EntryType], true); }
        }

        public static string ImageFolder
        {
            get { return ConfigurationManager.AppSettings[SettingKeys.ImageFolder]; }
        }
    }
}
