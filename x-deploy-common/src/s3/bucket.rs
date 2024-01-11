use crate::s3::config::CommonS3Config;
use crate::s3::file_type::ToFilePrefix;
use crate::CommonResult;
use rusoto_core::ByteStream;
use rusoto_s3::{
  DeleteObjectOutput, DeleteObjectRequest, DeleteObjectsOutput,
  DeleteObjectsRequest, PutObjectOutput, PutObjectRequest,
  ResponseContentTypeSerializer, S3,
};

pub struct CommonS3Bucket<T>
where
  T: ToFilePrefix,
{
  file_type: T,
  config: CommonS3Config,
}

impl<T> CommonS3Bucket<T>
where
  T: ToFilePrefix,
{
  pub fn new(
    file_type: T,
    config: CommonS3Config,
  ) -> Self {
    Self { file_type, config }
  }
}

impl<T> CommonS3Bucket<T>
where
  T: ToFilePrefix,
{
  pub async fn add(
    &self,
    filename: &String,
    data: &[u8],
    content_type: String,
  ) -> CommonResult<PutObjectOutput> {
    let bucket_name = self.config.bucket.clone();
    let bytes: ByteStream = ByteStream::from(data.to_vec());
    let filename = self.get_full_filename(&filename);

    let put_request = PutObjectRequest {
      bucket: bucket_name,
      key: filename,
      body: Some(bytes),
      content_type: Some(content_type),
      ..Default::default()
    };
    let result = self.config.s3_client.put_object(put_request).await?;
    Ok(result)
  }

  #[deprecated]
  pub async fn remove(
    &self,
    filename: &String,
  ) -> CommonResult<DeleteObjectOutput> {
    let bucket_name = self.config.bucket.clone();
    let filename = self.get_full_filename(&filename);
    let delete_request = DeleteObjectRequest {
      bucket: bucket_name,
      key: filename,
      ..Default::default()
    };
    let result = self.config.s3_client.delete_object(delete_request).await?;
    Ok(result)
  }

  pub fn get_public_url(
    &self,
    filename: &String,
  ) -> String {
    let endpoint = &self.config.endpoint;
    let bucket = &self.config.bucket;
    let filename = self.get_full_filename(&filename);
    format!("{}/{}/{}", endpoint, bucket, filename)
  }

  fn get_full_filename(
    &self,
    filename: &String,
  ) -> String {
    format!("{}-{}", self.file_type.to_prefix(), filename)
  }

  // #[deprecated]
  // pub async fn get(
  //   &self,
  //   filename: String,
  // ) -> CommonResult<Option<&[u8]>> {
  //   let get_request = rusoto_s3::GetObjectRequest {
  //     bucket: BUCKET_NAME.to_string(),
  //     key: filename,
  //     ..Default::default()
  //   };
  //   let result = self.config.s3_client.get_object(get_request).await?;
  //   let result_body = result.body;
  //   let result_body = match result_body {
  //     Some(body) => body,
  //     None => return Ok(None),
  //   };
  //   let byte_buffer: BytesBuffer = result_body.into_blocking_read();
  //   let result: &[u8] = result_body.
  //   Ok(())
  // }
}
