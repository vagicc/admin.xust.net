#[derive(Debug, Clone)]
pub struct ZhongHuaDianCangBook {
    pub book_name: String,        //书名
    pub book_author: String,      //作者
    pub front_cover: String,      //front_cover IS '书封面图';
    pub category: String,         //分类：名
    pub book_description: String, //书简介
    //SEO标题 SEO关键词 SEO描述
    pub seo_title: String,
    pub seo_keywords: String,
    pub seo_description: String,
    //书的章节数组 book_chapters  章节名，章节URL
    pub book_chapters: Vec<Chapter>,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub title: String, //章节名
    pub url: String,   //章节URL
}

#[derive(Debug, Clone)]
pub struct Chapters {
    pub title: String,     //章节标题
    pub content: String,   //章节内容
    pub book_name: String, //书籍名称
    //SEO标题 SEO关键词 SEO描述
    pub seo_title: String,
    pub seo_keywords: String,
    pub seo_description: String,
}

//取  章节标题， 章节内容， SEO三大， 取个书名
pub async fn zhdc_book_chapter_select(html: &str) -> Chapters {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};
    // let html = include_str!("src/views/reptile/中华典藏网书章节详情.html");  //这个用作测试
    let document = Document::from(html);
    let h1_node = document.find(Name("h1")).next().expect("找不到标签<h1>");
    let h1_title = h1_node.text().trim().to_string(); //
    log::debug!("章节标题：{}", h1_title);

    //章节内容：<div id="content" class="panel-body">
    let content_node = document
        .find(Attr("id", "content"))
        .next()
        .expect("章节见容ID：content");
    let content = content_node.text();
    log::debug!("章节内容详情：{}", content);

    // 处理SEO（tdk(t标题k关键词d描述）
    let seo_title_node = document.find(Name("title")).next().expect("SEO<title>");
    let seo_title = seo_title_node.text();
    let seo_title = seo_title.trim_end_matches("_中华典藏").to_string(); //去掉尾部
    let head_node = document.find(Name("head")).next().expect("");
    let keywords_node = head_node.find(Attr("name", "keywords")).next().unwrap();
    let keywords = keywords_node
        .attr("content")
        .expect("取得SEO关键字出错")
        .to_string();
    let description_node = head_node.find(Attr("name", "description")).next().unwrap();
    let seo_description = description_node
        .attr("content")
        .expect("取得SEO描述出错")
        .to_string();

    log::debug!(
        "SEO标题:{} \nSEO关键词：{:#?} \nSEO描述：{}",
        seo_title, //卷之二_运气易览_汪机_在线阅读_中华典藏
        keywords,
        seo_description
    );

    //取书名
    /*
    <h3 class="panel-title">
        <a href="https://www.zhonghuadiancang.com/xuanxuewushu/18783/">运气易览</a>
        <p class="pull-right"><a href="https://www.zhonghuadiancang.com/renwu/wangji2/">汪机作品集</a>
        </p>
    </h3>
    */
    let h3_node = document.find(Name("h3")).next().expect("找不到标签<h3>");
    let bookname_node = h3_node.find(Name("a")).next().expect("没书名");
    let book_name = bookname_node.text();
    log::debug!("书名：{}", book_name);

    Chapters {
        title: h1_title,      //章节标题
        content: content,     //章节内容
        book_name: book_name, //书籍名称
        //SEO标题 SEO关键词 SEO描述
        seo_title: seo_title,
        seo_keywords: keywords,
        seo_description: seo_description,
    }
}

/// 解析“中华典藏” ：https://www.zhonghuadiancang.com/
/// 书目录页：https://www.zhonghuadiancang.com/xueshuzaji/18289/
/// let html = include_str!("html/temp.html");
/// taobao_select(html).await;
pub async fn zhonghuadiancang_select(html: &str) -> ZhongHuaDianCangBook {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};

    // let html = include_str!("html/taobao.html");
    // let html = include_str!("src/views/reptile/temp.html");  //这个用作测试
    let document = Document::from(html);
    let h1_node = document.find(Name("h1")).next().expect("找不到标签<h1>");
    //<h1>中国道教史 <small>作者:<a href="https://www.zhonghuadiancang.com/renwu/yiming/">佚名</a></a></small></h1>
    let h1_title = h1_node.text().trim().to_string(); //这个标题还要处理
                                                      //H1标题：中国道教史  作者:佚名
                                                      //H1标题：百家姓注释  作者:佚名
                                                      //H1标题：运气易览  作者:汪机
                                                      //H1标题：素问灵枢心得  作者:胡文焕

    // println!("H1标题：{}", h1_title);
    let title_vec: Vec<&str> = h1_title.split("作者").collect();
    let title = title_vec
        .first()
        .expect("处理目录页文章标题出错")
        .trim()
        .to_string();
    log::debug!("处理后的标题=={:#?}==", title); //=="中国道教史"==
                                                 // let title = title_vec.nth(0); //取第一个元素

    let a_in_h1 = h1_node.find(Name("a")).next().unwrap();
    let author = a_in_h1.text().trim().to_string(); //这里作者正确
    log::debug!("H1作者：{}", author);

    //取得封面
    /*
    <div class="fmpic"><img
        src="https://www.zhonghuadiancang.com/d/file/142857df51c95c069a52184fc821810d.jpg"
        alt="中国道教史"></div>
    */
    let front_cover_node = document
        .find(Attr("class", "fmpic"))
        .next()
        .expect("找不到类：fmpic,处理书封面图");
    let front_cover_node = front_cover_node
        .find(Name("img"))
        .next()
        .expect("处理书封面图出错");
    let cover_img = front_cover_node
        .attr("src")
        .expect("取得图片URL出错")
        .to_string();
    log::debug!("书封面图：{}", cover_img);

    //书籍描述简介
    /*
    <p class="m-summary">提傅勤家著。傅勤家生卒年不详。生平履历不详。
        “中国文化史丛书”之一,1937年商务印书馆出版。1984年上海书店影印。全书共二十章,依次为:绪言、
        外人对于道教史之分期、诸书所述道教之起源、道之名义与其演变、道教以前之信仰、道教之形成
        、道教之神、道教之方术、道教之修养、道教之规律、道佛二教之互相利用、道佛二教之相排、
        唐宋两朝之道教、道教之流传海外、道教经典之编纂与焚毁、道教之分派、明清时代之道教、
        现在之道藏与辑要、宫观及道徒、结论。
    </p>
    */
    let description_node = document
        .find(Attr("class", "m-summary"))
        .next()
        .expect("找不到类：m-summary,处理书封面图");
    let book_description = description_node.text();
    log::debug!("书籍简介：\n{}", book_description);

    /*
    分类名：
    <div class="alert"
        style="background: #fff;border-left: none;border-right: none;border-radius: 0;padding: 10px 15px;margin-bottom: 0;border-color: #ebebeb;">
        <a href='https://www.zhonghuadiancang.com/tags-50-0.html' target='_blank'>近代</a> <a
            href='https://www.zhonghuadiancang.com/tags-136-0.html' target='_blank'>学术</a>
    </div>
     */
    let category_node = document
        .find(Attr("class", "alert"))
        .next()
        .expect("找不到类：alert,处理书分类");
    let category = category_node.text();
    log::debug!("抓取的分类名：{}", category);
    // let tem = category_node.find(Name(a));

    // 处理SEO（tdk(t标题k关键词d描述）
    let seo_title_node = document.find(Name("title")).next().expect("SEO<title>");
    let seo_title = seo_title_node.text();
    let seo_title = seo_title.trim_end_matches("_中华典藏").to_string(); //去掉尾部
    let head_node = document.find(Name("head")).next().expect("");
    let keywords_node = head_node.find(Attr("name", "keywords")).next().unwrap();
    let keywords = keywords_node
        .attr("content")
        .expect("取得SEO关键字出错")
        .to_string();
    let description_node = head_node.find(Attr("name", "description")).next().unwrap();
    let seo_description = description_node
        .attr("content")
        .expect("取得SEO描述出错")
        .to_string();

    log::debug!(
        "SEO标题:{} \nSEO关键词：{:#?} \nSEO描述：{}",
        seo_title,
        keywords,
        seo_description
    );

    //<ul class="list-group" id="booklist"> 章节目录
    // let imgs_ul = document.find(Attr("id", "J_UlThumb")).next();
    let book_node = document
        .find(Attr("id", "booklist"))
        .next()
        .expect("找不到目录ID：booklist");
    let mut chapters: Vec<Chapter> = Vec::new();

    for li_node in book_node.find(Name("li")) {
        /*
        <li class="list-group-item col-md-6 vv-book">
        <a href="https://www.zhonghuadiancang.com/xueshuzaji/18289/339342.html"
        title="第一章　绪　言">第一章　绪　言</a>
        </li>
         */
        let li_title = li_node.text().trim().to_string(); //第一章　绪　言
        log::debug!("章节目录：{}", li_title);

        let a_node = li_node.find(Name("a")).next().expect("msg");
        let url = a_node.attr("href").unwrap().to_string();
        log::debug!("目录链接：{}", url);
        let temp = Chapter {
            title: li_title,
            url: url,
        };
        chapters.push(temp);
    }

    let book = ZhongHuaDianCangBook {
        book_name: title,                   //书名
        book_author: author,                //作者
        front_cover: cover_img,             //front_cover IS '书封面图';
        category: category,                 //分类：名
        book_description: book_description, //书简介
        //SEO标题 SEO关键词 SEO描述
        seo_title: seo_title,
        seo_keywords: keywords,
        seo_description: seo_description,
        //书的章节数组 book_chapters  章节名，章节URL
        book_chapters: chapters,
    };

    book
}
//================================下面，都是以前没用的===============

#[derive(Debug, Clone)]
pub struct Reptile {
    pub title: String, //标题
    // pub list_img:String,  //封面图-列表图
    pub price_base: f64,       //起拍价
    pub current_price: String, //当前价
    pub assess_price: f64,     //评估价
    pub margin: f64,           //保证金
    pub start_time: i64,       //开拍时间
    pub end_time: i64,         //拍卖结束时间
    pub address: String,       //标的物地址
    pub disposal_unit: String, //处置单位:所属法院
    // pub external_url: String,  //拍卖url
    pub belong: i16,        //所属平台（1.淘宝、2.京东）
    pub stage: String,      //拍卖阶段（一拍、二拍、变卖、撤回）
    pub photos: Vec<Photo>, //相册
}

#[derive(Debug, Clone)]
pub struct Photo {
    pub external_small: String,
    pub external_middle: String,
    pub external_original: String,
}

pub fn get_jd_stage(paimai_times: i8) -> String {
    // 京东拍卖阶段   1一拍  2.二拍    4.变卖
    if paimai_times == 1 {
        return "一拍".to_string();
    }
    if paimai_times == 2 {
        return "一拍".to_string();
    }
    if paimai_times == 4 {
        return "一拍".to_string();
    }
    return "重拍".to_string();
}

/// 解析淘宝法拍车详情
/// let html = include_str!("html/taobao.html");
/// taobao_select(html).await;
pub async fn taobao_select(html: &str) -> Option<Reptile> {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};

    // let html = include_str!("html/taobao.html");
    let document = Document::from(html);

    let pm_main = document.find(Class("pm-main")).next();
    if pm_main.is_none() {
        log::error!("查无主体");
        println!("查无主体");
        return None;
    }
    let pm_main_node = pm_main.unwrap();
    let status_node = pm_main_node.find(Class("item-status")).next().unwrap();
    let status_string = status_node.text().trim().to_string();
    // println!("拍卖阶段-状态：{}", status_string);

    // let h1_node = pm_main_node.find(Name("h1")).next().unwrap();

    // id="J_ImgBooth" 第一张大图
    let title_node = document.find(Attr("id", "J_ImgBooth")).next().unwrap();
    let title = title_node.attr("alt").unwrap().trim();
    // println!("找到标题：{:?}", title);

    // id="J_UlThumb" 下为所有图片
    let imgs_ul = document.find(Attr("id", "J_UlThumb")).next();
    let imgs_ul_node = imgs_ul.unwrap();

    // 开始时间-结束时间  可以取得开始时间戳data-start与结束时间戳data-end ，但  ---
    /*
    <ul class="pm-bid-eyebrow">
        <li class="J_PItem" id="sf-countdown" data-itemId="675425091824" data-now="1657028404000"
            data-timeToEnd="1685996000" data-timeToStart="-3498004000" data-end="1658714400000"
            data-start="1653530400000">
            <span class="title J_TimeTitle">距结束</span>
            <span
            class="countdown J_TimeLeft"><var>0</var><em>天</em><var>0</var><var>0</var><em>时</em><var>0</var><em>分</em><var>0</var><em>秒</em></span>
            <span id="J_Delay" class="pm-delay"><em class="delayCnt">0</em>次延时</span>
        </li>
    */

    let mut start_time: i64 = 0;
    let mut end_time: i64 = 0;
    //拍卖结束后找不到此ID:sf-countdown
    if let Some(time_node) = document.find(Attr("id", "sf-countdown")).next() {
        let start_time_str = time_node
            .attr("data-start")
            .expect("找不到开始拍卖时间戳")
            .trim();
        start_time = start_time_str.parse::<i64>().unwrap();

        // println!("开始拍卖时间:{:#?}", start_time);
        let end_time_str = time_node
            .attr("data-end")
            .expect("拍卖结束时间戳找不到")
            .trim();
        end_time = end_time_str.parse::<i64>().unwrap();
        // println!("拍卖结束时间戳:{:#?}", end_time);
    }

    // let time_node = document.find(Attr("id", "sf-countdown")).next().unwrap();
    // let start_time = time_node
    //     .attr("data-start")
    //     .expect("找不到开始拍卖时间戳")
    //     .trim();
    // let start_time = start_time.parse::<i64>().unwrap();
    // // println!("开始拍卖时间:{:#?}", start_time);
    // let end_time = time_node
    //     .attr("data-end")
    //     .expect("拍卖结束时间戳找不到")
    //     .trim();
    // let end_time = end_time.parse::<i64>().unwrap();
    // // println!("拍卖结束时间戳:{:#?}", end_time);

    // 价格：id="sf-price"
    let price_node = document
        .find(Attr("id", "sf-price").descendant(Class("pm-current-price")))
        .next()
        .unwrap();
    let price_node = price_node.find(Name("em")).next().unwrap();
    let current_price = price_node.text().trim().replace(",", "");
    // println!("当前价：{}", current_price);

    let margin_node = document
        .find(Attr("id", "submitDeposit").descendant(Name("span")))
        .next();

    let mut margin = String::new();
    if let Some(margin_node) = margin_node {
        let margin_string = margin_node.text();
        let margin_split: Vec<&str> = margin_string.trim().split('¥').collect();
        margin = margin_split.last().expect("切割(保证金)出错").to_string();
    } else {
        println!("已拍卖结束，没未登录交保证HTML");
    }

    // 保证金 margin
    // let margin_node = document
    //     .find(Attr("id", "submitDeposit").descendant(Name("span")))
    //     .next()
    //     .expect("找不到保证金"); //J_HoverShow
    // let margin_string = margin_node.text();
    // let margin_split: Vec<&str> = margin_string.trim().split('¥').collect();

    // let margin = margin_split.last().expect("切割(保证金)出错");
    // println!("保证金:{}", margin);

    // 标的物位置
    let address_node = document.find(Attr("id", "itemAddress")).next().unwrap();
    let address = address_node.text();
    // println!("标的物位置-所在 省 市 区: {}", address);
    let address_node = document
        .find(Attr("id", "itemAddressDetail"))
        .next()
        .unwrap();
    let address_detail = address_node.text();
    // println!("标的物位置-详细地址: {}", address_detail);
    let address = format!("{} {}", address, address_detail);

    // 处置单位disposal_unit
    // let unit_node = document
    //     .find(Class("unit-name"))
    //     .next()
    //     .expect("处置单位disposal_unit找不到unit-name")
    //     .find(Name("a"))
    //     .next()
    //     .unwrap();
    let unit_node = document
        .find(Class("unit-org-content"))
        .next()
        .expect("处置单位找不到")
        .find(Name("p"))
        .next()
        .unwrap();
    let disposal_unit = unit_node.text().trim().to_string();
    // println!("处置单位:{}", disposal_unit);

    let mut assess_price = String::new();
    let mut price_base = String::new();

    // 评估价 assess_price
    let price = document.find(Attr("id", "J_HoverShow")).next().unwrap();
    for price_temp in price.find(Name("td")) {
        let price_title = price_temp.find(Class("pay-mark")).next().unwrap();
        let price_title = price_title.text();
        let price_number = price_temp.find(Class("J_Price")).next();
        if price_number.is_none() {
            // println!("路过,{}", price_title);
            continue;
        }
        let price_number = price_number.unwrap();
        let price_number = price_number.text().trim().to_string();
        let price_number = price_number.replace(",", "");
        println!("价格标题：{}，=> {}", price_title, price_number);
        if price_title.eq("评 估 价") {
            assess_price = price_number;
            continue;
        }
        // 起拍价 或 变卖价
        if price_title.eq("起拍价") || price_title.eq("变卖价") {
            price_base = price_number;
            continue;
        }
        // 这里再找“保证金”
        if margin.is_empty() && price_title.eq("保证金") {
            margin = price_number;
            continue;
        }
    }

    let mut photos_vec: Vec<Photo> = Vec::new();

    // 处理相册
    for li_node in imgs_ul_node.find(Name("li")) {
        /*
         <li class="pm-selected J_FristImg">
            <div class="pm-pic pm-s80 ">
                <a href="javascript:void(0);">
                <img src="//img.alicdn.com/bao/uploaded/i4/O1CN01qQAZj729a49qOiAGN_!!0-paimai.jpg_80x80.jpg"
                    alt="琼A7765D金杯牌小型普通客车" />
                </a>
            </div>
        </li>
        */
        let img_node = li_node.find(Name("img")).next().unwrap();
        let img_src = img_node.attr("src").unwrap();
        // println!("相册小图(80x80)：{}", img_src);
        let img_460 = img_src.replace("80x80", "460x460");
        // println!("相册大图(460x460)：{}", img_460);
        let img_960 = img_src.replace("80x80", "960x960");
        // println!("详情原图(960x960)：{}", img_960);
        // 相册小图(80x80)：  //img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_80x80.jpg
        // 相册大图(460x460)：//img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_460x460.jpg
        // 原图，在详情  https://img.alicdn.com/bao/uploaded/i1/O1CN01jKw1eA2CUyc16XOrd_!!0-paimai.jpg_960x960.jpg
        photos_vec.push(Photo {
            external_small: img_src.to_string(),
            external_middle: img_460,
            external_original: img_960,
        });
    }

    let price_base = price_base.parse::<f64>().expect("字符串转f64出错");
    // let assess_price = assess_price.parse::<f64>().expect("字符串转f64出错");
    let assess_price = assess_price.parse::<f64>().unwrap_or_default(); //无评估价时
    let margin = margin.trim().replace(",", "");
    let margin = margin.parse::<f64>().expect("字符串转f64出错");
    Some(Reptile {
        title: title.to_string(), //标题
        // pub list_img:String,  //封面图-列表图
        price_base: price_base,       //起拍价
        current_price: current_price, //当前价
        assess_price: assess_price,   //评估价
        margin: margin,               //保证金
        start_time: start_time,       //开拍时间
        end_time: end_time,           //拍卖结束时间
        address: address,             //标的物地址
        disposal_unit: disposal_unit, //处置单位:所属法院
        //  external_url: String,  //拍卖url
        belong: 1,            //所属平台（1.淘宝、2.京东）
        stage: status_string, //拍卖阶段（一拍、二拍、变卖、撤回）
        photos: photos_vec,
    })
}
