using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Xml.Serialization;

namespace MangaDb.Configurations
{
    [XmlRoot(ElementName = "group")]
    public class Group
    {
        [XmlAttribute(AttributeName = "id")]
        public int Id { get; set; }
        [XmlAttribute(AttributeName = "propName")]
        public string PropName { get; set; }
    }

    [XmlRoot(ElementName = "groups")]
    public class Groups
    {
        [XmlElement(ElementName = "group")]
        public List<Group> Group { get; set; }
    }

    [XmlRoot(ElementName = "entry")]
    public class Entry
    {
        [XmlAttribute(AttributeName = "regex")]
        public string Regex { get; set; }
        [XmlElement(ElementName = "groups")]
        public Groups Groups { get; set; }
    }

    [XmlRoot(ElementName = "mainConfig")]
    public class MainConfig
    {
        [XmlElement(ElementName = "listSiteUrl")]
        public string ListSiteUrl { get; set; }
        [XmlElement(ElementName = "entry")]
        public Entry Entry { get; set; }
        [XmlElement(ElementName = "recordRegex")]
        public string RecordRegex { get; set; }
        [XmlElement(ElementName = "dbFilePath")]
        public string FilePath { get; set; }

        public string GetLinkToPage(string partialLink)
        {
            Regex r = new Regex(@"(http.?:\/\/[\w.]*)");
            if (!r.IsMatch(partialLink))
                return r.Match(ListSiteUrl).Groups[0].Value + partialLink;
            return partialLink;
        }

    }
}
