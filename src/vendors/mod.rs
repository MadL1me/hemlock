// mod remote_url_vendor;
// pub mod download_providers;
// mod local_vendor;
// mod remote_git_blob_vendor;
// 
// use crate::remote_sources::SourceOrigin;
// use crate::remote_sources::SourceOrigin::GitLabUrl;
// use crate::vendors::remote_url_vendor::RemoteUrlVendor;
// use crate::VendorSource;
// // Способы скачки PathType:
// // - Просто URL ссылка
// // - Локальное копирование
// // - Git репозиторий (если это Glob)
// // - Какой-то другой особый протокол, WS к примеру
// 
// // 1. Создается некий Vendorer - штука которая понимает протокол
// // - Эту штуку мы получаем по SourceType
// //      -> Gitlab/Github -> RemoteGitVendor
// //      -> Local -> LocalFileVendor
// //      -> Ws -> WebsocketVendor и т.к
// 
// // Совокупность?
// //
// 
// // Датасорсы
// // - Github, Gitlab, Bitbucket
// // - local file system
// // - remote file system
// // - Pinterest, Youtube
// 
// type AnyError = Box<dyn std::error::Error>;
// 
// pub fn vendor_path(vendor_source: VendorSource) {
//     let vendor = get_vendorer(vendor_source.clone());
//     vendor.vendor(vendor_source).expect("TODO: panic message");
// }
// 
// pub fn get_vendorer(vendor_source: VendorSource) -> Box<dyn FileVendor> {
//     let source_type = SourceOrigin::match_by_path(&vendor_source.source);
// 
//     match source_type {
//         SourceOrigin::GitLabUrl => {
//             Box::new(RemoteUrlVendor{
//                 url_provider: Box::new(GitlabUrlProvider{}),
//             })
//         }
//         SourceOrigin::GitHubUrl => {
//             Box::new(RemoteUrlVendor{
//                 url_provider: Box::new(GithubUrlProvider{}),
//             })
//         }
//         _ => {
//             panic!("123")
//         }
//     }
// }
// 
// pub trait FileVendor {
//     fn vendor(&self, source: VendorSource) -> Result<&str, crate::AnyError>;
// }
