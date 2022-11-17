FROM rust:latest as build

RUN apt update -y && \
    apt install -y git apt-utils

RUN git clone https://github.com/rui314/mold.git && \
    mkdir mold/build && \
    cd mold/build && \
    git checkout v1.7.0 && \
    ../install-build-deps.sh && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. && \
    cmake --build . -j $(nproc) && \
    cmake --install .

WORKDIR /app/shiryobukai-uni

COPY . .

RUN mold -run cargo build --release


FROM gcr.io/distroless/cc

COPY --from=build /app/shiryobukai-uni/target/release/shiryobukai-uni /

EXPOSE 80

CMD [ "/shiryobukai-uni" ]
