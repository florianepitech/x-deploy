use crate::error::ApiError;
use image::{load_from_memory, DynamicImage, GenericImageView, ImageFormat};
use rocket::data::{ByteUnit, ToByteUnit};
use rocket::http::Status;
use rocket::Data;
use std::io::Cursor;

pub struct ProfilePicture {
  extension: ImageFormat,
  image: DynamicImage,
}

impl ProfilePicture {
  pub async fn from_data(data: Data<'_>) -> Result<ProfilePicture, ApiError> {
    let bytes = Self::read_data(data).await?;
    let image = Self::bytes_to_image(&bytes)?;
    let extension = Self::guess_format(&bytes)?;
    let result = ProfilePicture { extension, image };
    result.verify_valid_format()?;
    return Ok(result);
  }

  pub async fn from_bytes(bytes: Vec<u8>) -> Result<ProfilePicture, ApiError> {
    let image = Self::bytes_to_image(&bytes)?;
    let extension = Self::guess_format(&bytes)?;
    let result = ProfilePicture { extension, image };
    result.verify_valid_format()?;
    return Ok(result);
  }

  pub fn to_square(&self) -> Result<ProfilePicture, ApiError> {
    let (width, height) = self.image.dimensions();
    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;
    let square = self.image.crop_imm(x, y, size, size);
    return Ok(ProfilePicture {
      extension: self.extension,
      image: square,
    });
  }

  pub fn get_extension(&self) -> Result<String, ApiError> {
    let result = self.extension.extensions_str();
    return match result.get(0) {
      Some(extension) => Ok(extension.to_string()),
      None => {
        let message = "Fail to get extension of your file".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  pub fn get_image_bytes(&self) -> Result<Vec<u8>, ApiError> {
    let mut buffer: Vec<u8> = Vec::new();
    let result = self
      .image
      .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png);
    return match result {
      Ok(_) => Ok(buffer),
      Err(_) => {
        let message = "Fail to get bytes of your file".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  // Private

  fn verify_valid_format(&self) -> Result<(), ApiError> {
    let valid_formats = vec![
      ImageFormat::Png,
      ImageFormat::Jpeg,
      ImageFormat::Gif,
      ImageFormat::WebP,
    ];
    let is_valid = valid_formats.contains(&self.extension);
    return if is_valid {
      Ok(())
    } else {
      let message = "Invalid format of your file".to_string();
      Err(ApiError::new(Status::BadRequest, message))
    };
  }

  fn guess_format(bytes: &Vec<u8>) -> Result<ImageFormat, ApiError> {
    let format_result = image::guess_format(bytes);
    return match format_result {
      Ok(format) => Ok(format),
      Err(_) => {
        let message = "Fail to guess format of your file".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  fn bytes_to_image(bytes: &Vec<u8>) -> Result<DynamicImage, ApiError> {
    let image_result = load_from_memory(bytes);
    return match image_result {
      Ok(image) => Ok(image),
      Err(_) => {
        let message = "Fail to parse your file to image".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  async fn read_data(data: Data<'_>) -> Result<Vec<u8>, ApiError> {
    let data = data.open(10.megabytes()).into_bytes().await;
    return match data {
      Ok(data) => {
        if !data.is_complete() {
          let message = "Your file is too large".to_string();
          return Err(ApiError::new(Status::PayloadTooLarge, message));
        }
        let result = data.to_vec();
        Ok(result)
      }
      Err(_) => {
        let message = "Fail to read data of your file".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }
}
